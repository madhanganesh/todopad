import sqlite3
import random
from datetime import datetime, timedelta

# Connect to SQLite database
conn = sqlite3.connect("todopad.db")
cursor = conn.cursor()

# Define the start and end date
start_date = datetime(2024, 1, 1)
end_date = datetime(2025, 1, 31)  # Change as needed

# Function to generate a random effort value from 0.5 to 3.5 in steps of 0.5
def random_effort():
    return round(random.uniform(0.5, 3.5) / 0.5) * 0.5  # Ensures steps of 0.5

# Iterate over each date in the range
current_date = start_date
while current_date <= end_date:
    date_str = current_date.strftime("%Y-%m-%d")

    # Generate random effort values for both todos
    effort_1 = random_effort()
    effort_2 = random_effort()

    # Insert first todo with tag "Project-1"
    cursor.execute("""
        INSERT INTO todos (user_id, title, due, effort)
        VALUES (?, ?, ?, ?)
    """, (1, f"todo-{date_str}-Project-1", date_str, effort_1))
    
    # Get the last inserted todo_id
    todo_id_1 = cursor.lastrowid

    # Insert tag for first todo
    cursor.execute("""
        INSERT INTO tags (user_id, todo_id, tag)
        VALUES (?, ?, ?)
    """, (1, todo_id_1, "Project-1"))

    # Insert second todo with tag "Project-2"
    cursor.execute("""
        INSERT INTO todos (user_id, title, due, effort)
        VALUES (?, ?, ?, ?)
    """, (1, f"todo-{date_str}-Project-2", date_str, effort_2))
    
    # Get the last inserted todo_id
    todo_id_2 = cursor.lastrowid

    # Insert tag for second todo
    cursor.execute("""
        INSERT INTO tags (user_id, todo_id, tag)
        VALUES (?, ?, ?)
    """, (1, todo_id_2, "Project-2"))

    # Move to the next date
    current_date += timedelta(days=1)

# Commit and close connection
conn.commit()
conn.close()

print("Insertion complete!")
