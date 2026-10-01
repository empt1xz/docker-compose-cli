# 🐳 emptyxz-docker

> Gere arquivos `docker-compose.yml` para bancos de dados em poucos segundos.

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge\&logo=rust\&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge\&logo=docker\&logoColor=white)
![NPM](https://img.shields.io/badge/NPM-CB3837?style=for-the-badge\&logo=npm\&logoColor=white)
![Licença](https://img.shields.io/badge/Licença-MIT-red?style=for-the-badge)

</div>

---

## 📌 Sobre

O **emptyxz-docker** é uma CLI desenvolvida em **Rust** para gerar configurações do Docker Compose de forma interativa.

Você escolhe o banco, informa os dados e o arquivo `docker-compose.yml` é criado automaticamente.

## ⚡ Uso

Não precisa instalar globalmente.

```bash
npx emptyxz-docker create
```

Depois, escolha:

```text
🐳 Docker Compose Generator

1. PostgreSQL
2. MySQL
3. MongoDB

Escolha o banco:
```

Informe:

* Nome do banco
* Usuário
* Senha
* Porta

E pronto.

```text
docker-compose.yml criado com sucesso!
```

---

## 🗄️ Bancos suportados

### PostgreSQL

```yaml
services:
  postgres:
    image: postgres:17
    container_name: postgres
    restart: unless-stopped
    environment:
      POSTGRES_DB: mydb
      POSTGRES_USER: admin
      POSTGRES_PASSWORD: admin123
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data

volumes:
  postgres_data:
```

### MySQL

```yaml
services:
  mysql:
    image: mysql:8.4
    container_name: mysql
    restart: unless-stopped
    environment:
      MYSQL_DATABASE: mydb
      MYSQL_USER: admin
      MYSQL_PASSWORD: admin123
      MYSQL_ROOT_PASSWORD: root123
    ports:
      - "3306:3306"
    volumes:
      - mysql_data:/var/lib/mysql
```

### MongoDB

```yaml
services:
  mongodb:
    image: mongo:8
    container_name: mongodb
    restart: unless-stopped
    environment:
      MONGO_INITDB_DATABASE: mydb
      MONGO_INITDB_ROOT_USERNAME: admin
      MONGO_INITDB_ROOT_PASSWORD: admin123
    ports:
      - "27017:27017"
    volumes:
      - mongo_data:/data/db
```

---

## 🛠️ Tecnologias

* 🦀 Rust
* 🟢 Node.js
* 📦 NPM / NPX
* 🐳 Docker
* 📄 Docker Compose

O Rust é responsável pela CLI e geração do arquivo. O Node.js funciona como uma camada para distribuição através do NPM/NPX.

---

## 📂 Estrutura

```text
emptyxz-docker/
├── bin/
│   └── docker.js
├── binaries/
│   └── docker_cli.exe
├── package.json
└── README.md
```

---

## 🚀 Desenvolvimento

Clone o projeto:

```bash
git clone https://github.com/empt1xz/emptyxz-docker.git
```

Entre na pasta:

```bash
cd emptyxz-docker
```

Caso você seja usuário linux:

```bash
sudo dnf install mingw64-gcc (FEDORA)
sudo apt install gcc-mingw-w64-x86-64 (UBUNTU/DEBIAN)
```

```bash
rustup target add x86_64-pc-windows-gnu
```

Compile o Rust no Linux:

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

Compile o Rust no Windows:

```bash
cargo build --release
```

Teste:

```bash
node bin/docker.js create
```

---

## 🗺️ Próximos passos

* [x] PostgreSQL
* [x] MySQL
* [x] MongoDB
* [x] Portas personalizadas
* [x] Credenciais personalizadas
* [x] Volumes persistentes
* [x] CLI em Rust
* [x] Distribuição via NPX

---

## 📄 Licença

Este projeto está sob a licença **MIT**.

---

<div align="center">

Feito com 🦀 Rust por **emptyxz**

</div>
