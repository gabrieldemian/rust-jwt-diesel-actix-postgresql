# Overview
Rust web service with Actix web, Diesel and PostgreSQL.

Users can register and authenticate, using JWT Tokens, after that, they can access protected routes and post data.

## Configuration
1. Clone the project
2. Edit the file `.env` at the root of the directory with the following:

```bash
# Postgres database url
DATABASE_URL=

# (optional) How much minutes until JWT token expires (defaults to 20 minutes)
JWT_EXPIRATION_MINUTES=1

# (optional) The secret of the JWT
JWT_SECRET="very very secret password here"
```
3. Run the project `cargo run`.

## How to test
Now that you have the server running (on port 8080), you can send requests to the web api.
You can do that on the terminal using `curl`.

A few examples:

- Register
```bash
curl 127.0.0.1:8080/api/register \
   -X POST \
   -H 'content-type: application/json' \
   -d '{"username":"my_login","password":"my_password"}' -v
```

- Login
```bash
curl 127.0.0.1:8080/api/login \
   -X POST \
   -H 'content-type: application/json' \
   -d '{"username":"my_login","password":"my_password"}' -v
```

Login will give you a JWT token that you will use on the other routes.

Copy this token to your clipboard.

- Create a note: (This is how you can use the token on `curl`)

```bash
curl 127.0.0.1:8080/api/notes \
   -X POST \
   -H 'content-type: application/json' \
   -d '{"title":"how to create macros","content":"step 1: cry"}' \
   -H "Authorization: Bearer <your_token_here>" -v
```

- List all notes

```bash
curl 127.0.0.1:8080/api/notes \
   -X GET \
   -H 'content-type: application/json' \
   -H "Authorization: Bearer <your_token_here>" -v
```

- Update a note

```bash
curl 127.0.0.1:8080/api/notes/0 \
   -X PUT \
   -H 'content-type: application/json' \
   -d '{"title":"write macros","content":"bla bla bla"}' \
   -H "Authorization: Bearer <your_token_here>" -v
```

- Delete a note

```bash
curl 127.0.0.1:8080/api/notes/0 \
   -X DELETE \
   -H 'content-type: application/json' \
   -H "Authorization: Bearer <your_token_here>" -v
```

## How it works

The folder `migrations` contains sql scripts in the format required by Diesel.

The folder `src/routes` contains all the routes of the API, following their scope, for example: `/api/register` is located at: `src/routes/api/register.rs`

The folder `src/models` contains all the database models of the API including the database logic to handle their own data.

I wrote a middleware to handle JWT authentication only on authenticated routes.

I also created a simple bash script to generate schemas and write them on `src/schemas.rs`.

The script is `src/gen-schema.bash`.

The passwords of the users are encrypted using SHA-256.
