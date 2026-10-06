use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct DockerHubResponse {
    results: Vec<DockerTag>,
}

#[derive(Debug, Deserialize)]
struct DockerTag {
    name: String,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1).map(String::as_str) {
        Some("create") => init(),

        Some(command) => {
            println!("❌ Comando desconhecido: {}", command);
            println!();
            println!("Uso: docker-gen create");
        }

        None => {
            println!("🐳 Docker Compose Generator");
            println!();
            println!("Uso:");
            println!("  docker-gen create");
        }
    }
}

fn init() {
    println!("🐳 Docker Compose Generator\n");

    println!("1. PostgreSQL");
    println!("2. MySQL");
    println!("3. MariaDB");
    println!("4. MongoDB");
    println!("5. Redis");

    let choice = input("\nEscolha o banco: ");

    let (database, image_name) = match choice.as_str() {
        "1" => ("postgres", "postgres"),
        "2" => ("mysql", "mysql"),
        "3" => ("mariadb", "mariadb"),
        "4" => ("mongo", "mongo"),
        "5" => ("redis", "redis"),
        _ => {
            println!("❌ Opção inválida.");
            return;
        }
    };

    println!("\nBuscando versões disponíveis...");
    let tags = match fetch_docker_tags(image_name) {
        Ok(tags) => tags,
        Err(e) => {
            println!("Erro ao buscar versões: {}", e);
            println!("Usando versões padrão...\n");
            get_default_tags(database)
        }
    };

    println!("\nVersões disponíveis:");
    let display_count = tags.len().min(10);
    for (i, tag) in tags.iter().take(display_count).enumerate() {
        println!("{}. {}", i + 1, tag);
    }
    
    if tags.len() > display_count {
        println!("... e mais {} versões", tags.len() - display_count);
    }

    let version_choice = input(&format!("\nEscolha a versão (1-{}) ou digite manualmente: ", display_count));
    
    let version = if let Ok(idx) = version_choice.parse::<usize>() {
        if idx > 0 && idx <= tags.len() {
            tags[idx - 1].clone()
        } else {
            println!("❌ Número inválido. Usando versão padrão.");
            get_default_version(database)
        }
    } else if !version_choice.is_empty() {
        version_choice
    } else {
        println!("Usando versão padrão.");
        get_default_version(database)
    };

    println!("\nVersão selecionada: {}", version);

    let name = input("\nNome do banco: ");
    let username = input("Usuário: ");
    let password = input("Senha: ");

    let default_port = match database {
        "postgres" => "5432",
        "mysql" => "3306",
        "mariadb" => "3306",
        "mongo" => "27017",
        "redis" => "6379",
        _ => unreachable!(),
    };

    let port_input = input(&format!("Porta [{}]: ", default_port));

    let port = if port_input.is_empty() {
        default_port.to_string()
    } else {
        port_input
    };

    let content = match database {
        "postgres" => postgres_compose(&name, &username, &password, &port, &version),
        "mysql" => mysql_compose(&name, &username, &password, &port, &version),
        "mongo" => mongo_compose(&name, &username, &password, &port, &version),
        "mariadb" => mariadb_compose(&name, &username, &password, &port, &version),
        "redis" => redis_compose(&name, &username, &password, &port, &version),
        _ => {
            println!("❌ Banco inválido.");
            return;
        }
    };

    let path = Path::new("docker-compose.yml");

    if path.exists() {
        let overwrite = input("\n⚠️ docker-compose.yml já existe. Deseja substituir? (s/n): ");

        if overwrite.to_lowercase() != "s" {
            println!("❌ Operação cancelada.");
            return;
        }
    }

    match fs::write(path, content) {
        Ok(_) => {
            println!();
            println!("✅ docker-compose.yml criado com sucesso!");
            println!("📁 Local: {}", path.display());
        }
        Err(error) => {
            println!("❌ Erro ao criar o arquivo: {}", error);
        }
    }
}

fn fetch_docker_tags(image: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let url = format!(
        "https://registry.hub.docker.com/v2/repositories/library/{}/tags?page_size=100",
        image
    );
    
    let response = reqwest::blocking::get(&url)?;
    let data: DockerHubResponse = response.json()?;
    
    let mut tags: Vec<String> = data
        .results
        .into_iter()
        .map(|t| t.name)
        .filter(|name| {
            let lower = name.to_lowercase();
            name.chars().next().map(|c| c.is_numeric()).unwrap_or(false)
                && !lower.contains("windowsservercore")
                && !lower.contains("nanoserver")
                && name != "latest"
        })
        .collect();
    
    tags.sort_by(|a, b| {
        let a_has_alpine = a.contains("alpine");
        let b_has_alpine = b.contains("alpine");
        
        if a_has_alpine != b_has_alpine {
            return b_has_alpine.cmp(&a_has_alpine);
        }
        
        
        let a_parts: Vec<u32> = a.split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();
        let b_parts: Vec<u32> = b.split(|c: char| !c.is_numeric())
            .filter_map(|s| s.parse().ok())
            .collect();
        b_parts.cmp(&a_parts) 
    });
    
    Ok(tags)
}

fn get_default_tags(database: &str) -> Vec<String> {
    match database {
        "postgres" => vec!["17-alpine".to_string(), "17".to_string(), "16-alpine".to_string(), "16".to_string(), "15-alpine".to_string()],
        "mysql" => vec!["8.4".to_string(), "8.0".to_string(), "5.7".to_string()],
        "mariadb" => vec!["11".to_string(), "10".to_string()],
        "mongo" => vec!["8".to_string(), "7".to_string(), "6".to_string()],
        "redis" => vec!["8-alpine".to_string(), "8".to_string(), "7-alpine".to_string(), "7".to_string()],
        _ => vec!["latest".to_string()],
    }
}

fn get_default_version(database: &str) -> String {
    match database {
        "postgres" => "17-alpine".to_string(),
        "mysql" => "8.4".to_string(),
        "mariadb" => "11".to_string(),
        "mongo" => "8".to_string(),
        "redis" => "8-alpine".to_string(),
        _ => "latest".to_string(),
    }
}

fn input(message: &str) -> String {
    let mut value = String::new();
    print!("{}", message);
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut value).unwrap();
    value.trim().to_string()
}

fn postgres_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
    version: &str,
) -> String {
    format!(
        r#"services:
  postgres:
    image: postgres:{}
    container_name: postgres
    restart: unless-stopped
    environment:
      POSTGRES_DB: {}
      POSTGRES_USER: {}
      POSTGRES_PASSWORD: {}
    ports:
      - "{}:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
"#,
        version, name, username, password, port
    )
}

fn mysql_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
    version: &str,
) -> String {
    format!(
        r#"services:
  mysql:
    image: mysql:{}
    container_name: mysql
    restart: unless-stopped
    environment:
      MYSQL_DATABASE: {}
      MYSQL_USER: {}
      MYSQL_PASSWORD: {}
      MYSQL_ROOT_PASSWORD: {}
    ports:
      - "{}:3306"
    volumes:
      - mysql_data:/var/lib/mysql

volumes:
  mysql_data:
"#,
        version, name, username, password, password, port
    )
}

fn mongo_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
    version: &str,
) -> String {
    format!(
        r#"services:
  mongodb:
    image: mongo:{}
    container_name: mongodb
    restart: unless-stopped
    environment:
      MONGO_INITDB_DATABASE: {}
      MONGO_INITDB_ROOT_USERNAME: {}
      MONGO_INITDB_ROOT_PASSWORD: {}
    ports:
      - "{}:27017"
    volumes:
      - mongo_data:/data/db

volumes:
  mongo_data:
"#,
        version, name, username, password, port
    )
}

fn mariadb_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
    version: &str,
) -> String {
    format!(
        r#"services:
  mariadb:
    image: mariadb:{}
    container_name: mariadb
    restart: unless-stopped
    environment:
      MYSQL_DATABASE: {}
      MYSQL_USER: {}
      MYSQL_PASSWORD: {}
      MYSQL_ROOT_PASSWORD: {}
    ports:
      - "{}:3306"
    volumes:
      - mariadb_data:/var/lib/mysql

volumes:
  mariadb_data:
"#,
        version, name, username, password, password, port
    )
}

fn redis_compose(
    name: &str,
    username: &str,
    password: &str,
    port: &str,
    version: &str,
) -> String {
    format!(
        r#"services:
  redis:
    image: redis:{}
    container_name: redis
    restart: unless-stopped
    command: redis-server --requirepass {}
    ports:
      - "{}:6379"
    volumes:
      - redis_data:/data

volumes:
  redis_data:
"#,
        version, password, port
    )
}
