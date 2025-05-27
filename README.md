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
git clone https://github.com/zionariunbold/rust-api.git
cd rust-api

# Сангуудыг татаж авах
cargo build

# Migration хийх (Хэрвээ Diesel ашиглаж байгаа бол)
diesel setup
diesel migration run

# Төслийн root хавтас дотор .env файл үүсгээд дараах утгуудыг тохируулна:
DATABASE_URL=postgres://postgres:yourpassword@127.0.0.1:5432/your_db
SECRET_KEY=your_jwt_secret_key

# Rocket framework-ийн тохиргоог Rocket.toml файлд дараах байдлаар хийнэ:
[default.databases.postgres]
url = "postgres://postgres:yourpassword@127.0.0.1:5432/your_db"

🚀 Серверийг ажиллуулах
cargo run
