# 🚀 MiniServe - Rust API Server

MiniServe adalah sebuah API server yang dibangun menggunakan Rust dengan framework Actix Web dan database SQLite. Aplikasi ini menyediakan fitur-fitur dasar seperti registrasi, login, manajemen pengguna, dan autentikasi menggunakan JWT (JSON Web Token).

---

## 🌟 Fitur Utama

- **Registrasi Pengguna**: Buat akun baru dengan nama, email, dan sandi.
- **Login Pengguna**: Dapatkan token JWT untuk akses terproteksi.
- **Manajemen Pengguna**:
  - Baca daftar pengguna.
  - Perbarui informasi pengguna.
  - Hapus pengguna.
- **Autentikasi JWT**: Proteksi endpoint dengan token JWT.

---

## 🛠️ Teknologi yang Digunakan

- **Bahasa Pemrograman**: Rust
- **Framework Web**: Actix Web
- **Database**: SQLite
- **Autentikasi**: JWT (JSON Web Token)
- **Hashing Password**: Argon2
- **Environment Management**: dotenv
- **Logging**: env_logger

---

## 🚀 Cara Menjalankan Proyek

### Prasyarat

- **Rust**: Pastikan Rust terinstal. Jika belum, instal dari [rustup.rs](https://rustup.rs).
- **SQLx CLI**: Instal SQLx CLI untuk migrasi database:
  ```bash
  cargo install sqlx-cli
  ```

### Langkah-Langkah

1. **Clone Repositori**:
   ```bash
   git clone https://github.com/username/miniserve.git
   cd miniserve
   ```

2. **Setup Environment**:
   Buat file `.env` di root direktori dan isi dengan:
   ```bash
   JWT_KEY="secret_key"
   JWT_DURASI=10000  # 10000 detik
   DATABASE_URL="imphnen.db"
   ```

3. **Jalankan Migrasi Database**:
   ```bash
   sqlx database create
   sqlx migrate run
   ```

4. **Jalankan Server**:
   ```bash
   cargo run
   ```
   Server akan berjalan di `http://127.0.0.1:8080`.

---

## 📚 Dokumentasi API

### 1. Registrasi Pengguna
**Endpoint**: `POST /api/public/signup`

**Body:**
```json
{
  "nama": "Riky Ripaldo",
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}
```

### 2. Login Pengguna
**Endpoint**: `POST /api/public/signin`

**Body:**
```json
{
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}
```

### 3. Baca Daftar Pengguna
**Endpoint**: `GET /api/private/users`

**Header:**
```bash
Authorization: Bearer <JWT Token>
```

### 4. Perbarui Informasi Pengguna
**Endpoint**: `PUT /api/private/users/:id`

**Header:**
```bash
Authorization: Bearer <JWT Token>
```

**Body:**
```json
{
  "nama": "Riky Ripaldo",
  "email": "riky@imphnen.malas",
  "sandi": "321#nimdA"
}
```

### 5. Hapus Pengguna
**Endpoint**: `DELETE /api/private/users/:id`

**Header:**
```bash
Authorization: Bearer <JWT Token>
```

**Body (Optional):**
```json
{
  "nama": "Riky Ripaldo",
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}
```

---

## 🧑‍💻 Contoh Penggunaan

### Registrasi Pengguna
```bash
curl -X POST http://localhost:8080/api/public/signup \
-H "Content-Type: application/json" \
-d '{
  "nama": "Riky Ripaldo",
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}'
```

### Login Pengguna
```bash
curl -X POST http://localhost:8080/api/public/signin \
-H "Content-Type: application/json" \
-d '{
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}'
```

### Baca Daftar Pengguna
```bash
curl -X GET http://localhost:8080/api/private/users \
-H "Authorization: Bearer <JWT Token>"
```

---

## 🛡️ Keamanan

- **Password Hashing**: Sandi pengguna di-hash menggunakan Argon2.
- **JWT**: Autentikasi menggunakan token JWT dengan waktu ekspirasi.
- **Environment Variables**: Konfigurasi sensitif disimpan di file `.env`.
- **Custom Logging**: Log hanya mencatat informasi penting seperti IP, URL, Method, Speed, Request Code.

---

## 🤝 Berkontribusi

Kami sangat menerima kontribusi! Jika Anda ingin berkontribusi, silakan buka issue atau kirim pull request.

---

## 📄 Lisensi

Proyek ini dilisensikan di bawah MIT License. Lihat file LICENSE untuk detail lebih lanjut.

---

## 🙏 Terima Kasih

Terima kasih telah menggunakan MiniServe! Jika Anda memiliki pertanyaan atau masukan, silakan buka issue atau hubungi kami.
