sqlx database create
sqlx migrate run

flyctl secrets set DATABASE_URL=sqlite:///data/todopad.db SQLX_OFFLINE=true RUST_LOG=debug

