cargo install sqlx-cli
export DATABASE_URL=sqlite://todo.db
sqlx database create
sqlx migrate run

echo "~/.cargo/bin/cargo sqlx prepare 2>&1 >/dev/null; git add .sqlx" > .git/hooks/pre-commit

flyctl secrets set ENV=release DATABASE_URL=sqlite:///data/todopad.db SQLX_OFFLINE=true RUST_LOG=debug GEMINI_API_KEY=AIzaSyBMREWluJ1x7m5IaHSXL_2AKXEORknO8bY

flyctl secrets set DATABASE_URL=sqlite://todopad.db SQLX_OFFLINE=true RUST_LOG=debug GEMINI_API_KEY=AIzaSyBMREWluJ1x7m5IaHSXL_2AKXEORknO8bY


# Connect SQLite DB in fly.io
flyctl ssh console
apt-get update && apt-get install -y sqlite3

docker build -t todopad .

docker run --name todopad -e ENV=release -e DATABASE_URL=sqlite://todopad.db -e SQLX_OFFLINE=true -e RUST_LOG=error -e OPENAI_API_KEY=<key> -p 8080:8080 todopad


--env-file .env

fly ssh console -a todopad  


TODO:
0. Edit Insight [Done]
    Delete Insight [Done]
    After insight creation navigate to that insight [Done]
    When adding tags 
    a. message to press enter [Done]
    b. selection of existing tags by arrows keys and enter [Done]
    c. autocompletion tags pop-up position [Done]
    c. adjust the chart in mobile [Done]


bugs
====
[Done] Bar Chart and Pie Graps are coming wrong
[Done] On completipn should not remove other than Pending

1. when editing the title the tags are not regenerated. but when implementing this, need to careful
that if user has given any tags explicitly - then no need to regenarate the tags again
[ Done ]]

2. Show data (relative day) in the list
[Done]

3. Parse any link in the notes and show links and open from todo item [Done] 

4. Click on link icon is confusing esp i there is only one link

5. [Done] Run google page insights - and fix the perf issues

6. [Done] Timezone issues... if I create in US, singapore date is used

7. [Open] add UI build also to Docker

7. Secret key is hard coded - handlers/mod.rs

8. WARN call:get_record: tower_sessions_core::session: possibly suspicious activity: record not found in store

9. Color code on due dates that are passed

10. logging with error (and alerts)

11. Charts
    a. count metric in charts
    b. Show description of insight

12. Refactor edit HTML

13. [Done] move JS to files and minify

14. Further speed improvement (googl epage insights)


