# Commands
goose -dir ./_db/migrations sqlite3 _db/todopad.sqlite up


# TODO

* Responsive UI to mobile
* Handle token expiration case
* Forget Password
* Login/Signup scree - error stays as user start fixing it
* Fix: user not found but adding todo (Internal Error. Try later or check with admin.: error in repository.Todo::Create when inserting a todo: pq: insert or update on table "todos" violates foreign key constraint "fk_user")
