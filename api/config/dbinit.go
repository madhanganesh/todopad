package config

import "database/sql"

func GetSqliteDB(path string) (*sql.DB, error) {
	return sql.Open("sqlite3", path+"?_foreign_keys=on")
}
