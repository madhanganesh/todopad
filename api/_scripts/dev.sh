export DATABASE_URL="user=postgres password=zenith sslmode=disable"
export MIGRATIONS_DIR=./_scripts/migrations
export SECRET_KEY=@wedidit#foryou*
export PORT=8080

if [[ -z "$MIGRATIONS_DIR" ]]; then
    echo "Must provide MIGRATIONS_DIR in environment" 1>&2
    exit 1
fi

goose --dir $MIGRATIONS_DIR postgres $DATABASE_URL status 
goose --dir $MIGRATIONS_DIR postgres $DATABASE_URL up
