# 🚀 Rust + Rocket Web API

Энэ төсөл нь [Rocket](https://rocket.rs/) framework ашиглан бүтээгдсэн RESTful API бөгөөд PostgreSQL өгөгдлийн санд холбогдож, JWT токен ашиглан authentication хийдэг.

---

## 📦 Шаардлагатай зүйлс

- Rust (edition 2021+)
- Cargo
- PostgreSQL сервер
- Diesel CLI (`cargo install diesel_cli --no-default-features --features postgres`)
- `.env` файл
- `Rocket.toml` файл

---

## ⚙️ Төслийг суулгах алхмууд

```bash
# Төслийг хуулж авах
git clone https://github.com/yourusername/yourproject.git
cd yourproject

# Зависимуудыг татаж авах
cargo build

# Migration хийх (Хэрвээ Diesel ашиглаж байгаа бол)
diesel setup
diesel migration run

## 🛠 Rocket.toml тохиргоо
Rocket framework-ийн тохиргоог Rocket.toml файлд дараах байдлаар хийнэ:
```bash
[default.databases.postgres]
url = "postgres://postgres:yourpassword@127.0.0.1:5432/your_db"
