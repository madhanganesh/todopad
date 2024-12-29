sqlx database create
sqlx migrate run

echo "~/.cargo/bin/cargo sqlx prepare -- --lib 2>&1 >/dev/null; git add sqlx-data.json" > .git/hooks/pre-commit

flyctl secrets set DATABASE_URL=sqlite:///data/todopad.db SQLX_OFFLINE=true RUST_LOG=debug

# Connect SQLite DB in fly.io
flyctl ssh console
apt-get update && apt-get install -y sqlite3

