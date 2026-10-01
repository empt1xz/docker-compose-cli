<h1 style="font-size: 48px"> 🐳 <b>emptyxz-docker</b> </h1>

> Generate `docker-compose.yml` files for databases in seconds.

<div align="center">

</div>

---

### 📌 About

##### **emptyxz-docker** is a CLI built with **Rust** that interactively generates Docker Compose configurations.
###### You pick the database, enter the details, and the `docker-compose.yml` file is created automatically.
---
### ⚡ **Usage**

#### No global installation required.

```bash
npx emptyxz-docker create
```

#### Then choose:

```text
🐳 Docker Compose Generator

1. PostgreSQL
2. MySQL
3. MongoDB

Choose the database:
```

#### Enter:

* Database name
* User
* Password
* Port

#### And you're done.

```text
docker-compose.yml created successfully!
```

---

### 🗄️ *Supported databases*

### **PostgreSQL**

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

### **MySQL**

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

### **MongoDB**

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

## 🛠️ Technologies

* 🦀 [Rust](https://rust-lang.org/)
- 🟢 [Node.js](https://nodejs.org/)
* 📦 [NPM / NPX](https://npmjs.com/)
- 🐳 [Docker](https://www.docker.com/)
* 📄 [Docker Compose](https://docs.docker.com/compose/)

###### *Rust* handles the *CLI* and file generation. *Node.js* acts as a layer for distribution through *NPM/NPX*.

---

### 📂 **Structure**

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

### 🚀 **Development**

### **Step 1** - Clone the project:

```bash
git clone https://github.com/empt1xz/emptyxz-docker.git
```

### **Step 2** - Enter the folder:

```bash
cd emptyxz-docker
```

### **Step 3** - Build the Rust binary:

```bash
cargo build --release
```

### **Step 4** - Test it:

```bash
node bin/docker.js create
```

<div align="center">

### Made with 🦀 by **emptyxz**

</div>

<br>

<div align="center">

![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge\&logo=rust\&logoColor=white)
![Docker](https://img.shields.io/badge/Docker-2496ED?style=for-the-badge\&logo=docker\&logoColor=white)
![NPM](https://img.shields.io/badge/NPM-CB3837?style=for-the-badge\&logo=npm\&logoColor=white)

![License](https://img.shields.io/badge/License-MIT-red?style=for-the-badge)

</div>