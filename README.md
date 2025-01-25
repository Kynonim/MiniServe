# MiniServe
* MiniServer Rust

### .env
```bash
JWT_KEY = "secret_key"
JWT_DURASI = 10000 # 10000 detik = 10000/60 = 166.66666666666667 menit
DATABASE_URL = "imphnen.db"
```

# API
### Create Account
* POST /api/public/signup
```json
{
  "nama": "Riky Ripaldo",
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}
```

### Login Account
* POST /api/public/sigin
```json
{
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}
```

### Read Users
* GET /api/private/users
> JWT Token Needed
> Authorization: Bearer <JWT Token>

### Update Account
* PUT /api/private/users/:id
> JWT Token Needed
> Authorization: Bearer <JWT Token>
```json
{
  "nama": "Riky Ripaldo",
  "email": "riky@imphnen.malas",
  "sandi": "321#nimdA"
}
```

### Create Account
* DELETE /api/private/users/:id
> JWT Token Needed
> Authorization: Bearer <JWT Token>
```json
{
  "nama": "Riky Ripaldo",
  "email": "riky@imphnen.scroll",
  "sandi": "Admin#123"
}
```