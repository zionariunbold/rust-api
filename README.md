# 🚀 Rust + Rocket Web API

Энэ төсөл нь [Rocket](https://rocket.rs/) framework ашиглан хийгдсэн RESTful API бөгөөд PostgreSQL өгөгдлийн санд холбогддог. Төслийн код нь asynchronous, secure бөгөөд JWT токен ашиглан authentication хийдэг.

---

## ⚙️ Суулгалт

```bash
git clone https://github.com/zionariunbold/rust-api.git
cd yourproject
cargo build


📁 .env тохиргоо
Төслийн root-д .env файл үүсгээд дараах утгуудыг оруулна:

env
Copy
DATABASE_URL=postgres://postgres:yourpassword@127.0.0.1:5432/your_db
SECRET_KEY=your_jwt_secret_key
DATABASE_URL – PostgreSQL холболтын мөр

SECRET_KEY – JWT токен үүсгэхэд ашиглагддаг нууц түлхүүр
(32 тэмдэгтээс урт, санамсаргүй утгатай байвал сайн)

🛠 Rocket.toml тохиргоо
Rocket.toml файлд өгөгдлийн сангийн тохиргоог дараах байдлаар хийнэ:

toml
Copy
[default.databases.postgres]
url = "postgres://postgres:yourpassword@127.0.0.1:5432/your_db"

🚀 API-г ажиллуулах
bash
Copy
cargo run
Сервер http://localhost:8000 дээр ажиллана.

🔐 Authentication (Нэвтрэлт баталгаажуулалт)
JWT (JSON Web Token) ашиглан хэрэглэгч бүртгэл болон нэвтрэлтийг баталгаажуулдаг. Нууц түлхүүр нь .env файлаас авна.

🧾 Хамааралтай сангууд
rocket - Web framework

diesel + diesel_async - ORM & async query

dotenvy - .env дэмжигч

jsonwebtoken - JWT үүсгэгч
