
if [[ -z "$SQLITE_DIR" ]]; then
    echo "Must provide SQLITE_DIR in environment" 1>&2
    exit 1
fi

goose --dir ./_db/migrations sqlite3 $SQLITE_DIR/todopad.sqlite status 
goose --dir ./_db/migrations sqlite3 $SQLITE_DIR/todopad.sqlite up