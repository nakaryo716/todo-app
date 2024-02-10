# Todo app
This is Todo App by using rust language.  
The framework that this app using is Axum.
Client side framework is React(TypeScript)

# Run
To use database, We build docker container.
```
docker compose up
```
Next, running backend.
```
cd api/
sqlx database create
sqlx migrate run
cargo run
```
Finaly, run clientside
```
cd client/
npm i
npm run dev
```
Open http://localhost:3000 to view it in the browser.

