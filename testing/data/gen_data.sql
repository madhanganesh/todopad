-- Generate all working days (Monday to Friday) from 2024-06-01 to tomorrow
WITH RECURSIVE date_series AS (
    SELECT DATE('2024-06-01') AS due_date
    UNION ALL
    SELECT DATE(due_date, '+1 day')
    FROM date_series
    WHERE due_date < DATE('now', '+5 day')
)
SELECT due_date FROM date_series WHERE STRFTIME('%w', due_date) NOT IN ('0', '6');

-- Insert 4 todos per working day
INSERT INTO todos (user_id, title, due, completed, notes)
SELECT 
    1, 
    'Task ' || (ROW_NUMBER() OVER (PARTITION BY due_date ORDER BY due_date)) || ' - ' || due_date, 
    due_date, 
    CASE 
        WHEN due_date >= DATE('now', '-14 days') THEN 0 
        ELSE 1 
    END, 
    'Auto-generated task'
FROM (
    SELECT due_date FROM (
        WITH RECURSIVE date_series AS (
            SELECT DATE('2024-06-01') AS due_date
            UNION ALL
            SELECT DATE(due_date, '+1 day')
            FROM date_series
            WHERE due_date < DATE('now', '+1 day')
        )
        SELECT due_date FROM date_series WHERE STRFTIME('%w', due_date) NOT IN ('0', '6')
    ) dates
    CROSS JOIN (SELECT 1 AS num UNION ALL SELECT 2 UNION ALL SELECT 3 UNION ALL SELECT 4)
) AS expanded_dates;

-- Insert tags for first 2 todos of each day (Project-1 related)
INSERT INTO tags (user_id, todo_id, tag)
SELECT 
    1, 
    id, 
    tag 
FROM todos
JOIN (
    SELECT 'Project-1' AS tag UNION ALL
    SELECT 'Design' UNION ALL
    SELECT 'Technical'
) AS tag_list 
ON todos.id % 4 < 2;

-- Insert tags for last 2 todos of each day (Project-2 related)
INSERT INTO tags (user_id, todo_id, tag)
SELECT 
    1, 
    id, 
    tag 
FROM todos
JOIN (
    SELECT 'Project-2' AS tag UNION ALL
    SELECT 'Coordination' UNION ALL
    SELECT 'Management'
) AS tag_list 
ON todos.id % 4 >= 2;
