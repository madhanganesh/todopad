use std::collections::HashMap;

use sqlx::{SqlitePool, query_as, Row, Error as SqlxError};
use thiserror::Error;

use crate::models::Insight;

#[derive(Debug, Error)]
pub enum InsightSaveError {
    #[error("Insight name already exists")]
    InsightNameAlreadyExists,

    #[error("Database error")]
    DatabaseError(#[from] sqlx::Error),
}  


pub async fn get_insights(pool: &SqlitePool, user_id: i64) 
    -> Result<Vec<Insight>, sqlx::Error> 
{
    let mut insights = query_as!(Insight, "SELECT * FROM insights WHERE user_id = ?", user_id)
                        .fetch_all(pool)
                        .await?;

    if insights.is_empty() {
        query_as!(Insight, 
            r#"INSERT INTO insights (user_id, name, description, metric, chart_type, periods) 
            VALUES (?, ?, ?, ?, ?, ?)"#,
            user_id, 
            "Efforts",
            "Efforts spent in a given period",
            "effort",
            "line",
            "Daily, Weekly, Monthly"
        )
        .execute(pool)
        .await?;

        insights = query_as!(Insight, "SELECT * FROM insights WHERE user_id = ?", user_id)
                    .fetch_all(pool)
                    .await?;
    }

    Ok(insights)
}

pub async fn create_insight(pool: &SqlitePool, insight: &Insight) 
    -> Result<Insight, InsightSaveError> 
{
    let result = query_as!(Insight, 
            r#"INSERT INTO insights (user_id, name, description, metric, chart_type, periods, tags) 
            VALUES (?, ?, ?, ?, ?, ?, ?)"#,
            insight.user_id, 
            insight.name,
            insight.description,
            insight.metric,
            insight.chart_type,
            "Daily, Weekly, Monthly",
            insight.tags,
        )
        .execute(pool)
        .await
        .map_err(map_sqlx_error)?;

    let last_insert_id = result.last_insert_rowid();
    let insight = sqlx::query_as!(
        Insight,
        "SELECT * FROM insights WHERE id = ?",
        last_insert_id
    )
    .fetch_one(pool)
    .await?;


    Ok(insight)
}

pub async fn update_insight(pool: &SqlitePool, insight: &Insight) 
    -> Result<Insight, InsightSaveError> 
{
    query_as!(Insight, 
            r#"UPDATE insights SET name=?, chart_type=?, metric=?, tags=?, description=?
            where user_id=? AND id=?"#,
            insight.name,
            insight.chart_type,
            insight.metric,
            insight.tags,
            insight.description,
            insight.user_id,
            insight.id
    )
    .execute(pool)
    .await
    .map_err(map_sqlx_error)?;

    let insight = sqlx::query_as!(
        Insight,
        "SELECT * FROM insights WHERE id = ?",
        insight.id    
    )
    .fetch_one(pool)
    .await?;

    Ok(insight)
}

pub async fn get_insight(pool: &SqlitePool, user_id: i64, insight_id: i64) 
    -> Result<Insight, sqlx::Error> {

    let insight = query_as!(Insight, 
        "SELECT * FROM insights WHERE user_id = ? AND id=?", user_id, insight_id
        )
        .fetch_one(pool)
        .await?;

    Ok(insight)
}

pub async fn delete_insight(pool: &SqlitePool, user_id: i64, insight_id: i64) 
    -> Result<(), sqlx::Error> {

    query_as!(Todo, "DELETE from insights where user_id=? and id=?", user_id, insight_id)
        .execute(pool)
        .await?;

    Ok(()) 
}

pub async fn get_efforts(
    pool: &SqlitePool, 
    user_id: i64, 
    filter: &str
) -> Result<(Vec<String>, HashMap<String, Vec<f64>>), sqlx::Error> {
   let rows = sqlx::query(get_effort_query(filter).as_str())
        .bind(user_id)
        .fetch_all(pool)
        .await
        .unwrap();

    let mut labels = vec![];
    let mut values: HashMap<String, Vec<f64>> = HashMap::new();

    for row in rows {
        labels.push(row.get("period"));
        values.entry("Efforts".to_string())
            .or_default()
            .push(row.get("value"));
    }

    Ok((labels, values))
}

pub async fn get_efforts_by_tags(
    pool: &SqlitePool, 
    user_id: i64, 
    filter: &str,
    tags: &str,
) -> Result<(Vec<String>, HashMap<String, Vec<f64>>), sqlx::Error> { 
    if tags.is_empty() {
        return Err(sqlx::Error::Protocol("No tags provided".into()));
    }

    let tags_vec: Vec<&str> = tags.split(",").collect();
    let placeholders = tags_vec.iter().map(|_| "?").collect::<Vec<_>>().join(", ");
    let query =  get_effort_grouped_by_tags_query(filter, &placeholders);
    let mut query_builder = sqlx::query(&query);
    query_builder = query_builder.bind(user_id);
    for tag in &tags_vec {
        query_builder = query_builder.bind(tag);
    }

    let rows = query_builder
        .fetch_all(pool)
        .await
        .unwrap();

    let mut labels: Vec<String> = Vec::new();
    let mut seen_labels = std::collections::HashSet::new();
    let mut values: HashMap<String, Vec<f64>> = HashMap::new();
    for row in rows {
        let label: String = row.get("period");
        if seen_labels.insert(label.clone()) {
            labels.push(label);
        }
        let key: String = row.get("tag");
        values.entry(key).or_default().push(row.get("value"));
    }

    Ok((labels, values))
}


fn map_sqlx_error(err: SqlxError) -> InsightSaveError {
    if let SqlxError::Database(db_err) = &err {
        if db_err.message().contains("UNIQUE constraint failed") {
            return InsightSaveError::InsightNameAlreadyExists;
        }
    }
    InsightSaveError::DatabaseError(err)
}

fn get_effort_query(filter: &str) -> String {
        match filter {
            "daily" => {
                format!(r#"
                    SELECT {} AS period, SUM(todos.effort) AS value
                    FROM todos 
                    WHERE due BETWEEN {}
                    AND user_id = ?
                    GROUP BY period
                    ORDER BY todos.due;
                "#, for_day(), for_day_range())
            }
            "weekly" => {
                format!(r#"
                    SELECT {} AS period, SUM(todos.effort) AS value
                    FROM todos 
                    WHERE due BETWEEN {}
                    AND user_id = ?
                    GROUP BY period
                    ORDER BY todos.due;
                "#, for_week(), for_week_range())
            }
            _ => {
                format!(r#"
                    SELECT {} as period,
                    SUM(todos.effort) AS value
                    FROM todos 
                    WHERE due BETWEEN {}
                    AND user_id = ?
                    GROUP BY period
                    ORDER BY todos.due;
                "#, for_month(), for_month_range())
            }
        }
}


fn get_effort_grouped_by_tags_query(filter: &str, tags_place_holder: &str) -> String {
        match filter {
            "daily" => {
                format!(r#"
                    SELECT {} AS period, tags.tag AS tag, sum(todos.effort) AS value
                    from todos as todos join tags as tags on tags.todo_id = todos.id
                    WHERE due BETWEEN {}
                    and todos.user_id=?
                    and tags.tag in ({})
                    GROUP BY tags.tag, period
                    ORDER BY todos.due;
                "#, for_day(), for_day_range(), tags_place_holder)
            }
            "weekly" => {
                format!(r#"
                    SELECT {} AS period, tags.tag, SUM(todos.effort) AS value
                    FROM todos as todos join tags as tags on tags.todo_id = todos.id
                    WHERE todos.due BETWEEN {}
                    AND todos.user_id = ?
                    and tags.tag in ({})
                    GROUP BY tags.tag, period
                    ORDER BY todos.due;
                "#, for_week(), for_week_range(), tags_place_holder)
            }
            _ => {
                format!(r#"
                    SELECT {} AS period, tags.tag, SUM(todos.effort) AS value
                    FROM todos join tags on tags.todo_id = todos.id
                    WHERE todos.due BETWEEN {}
                    AND todos.user_id = ?
                    and tags.tag in ({})
                    GROUP BY tags.tag, period  
                    ORDER BY todos.due;
                "#, for_month(), for_month_range(), tags_place_holder)
            }
        }
}

fn for_day() -> &'static str {
    "STRFTIME('%d', todos.due) || ' ' || SUBSTR('JanFebMarAprMayJunJulAugSepOctNovDec', 1 + 3 * (STRFTIME('%m', todos.due) - 1), 3)"
}

fn for_day_range() -> &'static str {
    "DATE('now', 'weekday 0', '-6 days') AND DATE('now', 'weekday 0', '6 days')"
}

fn for_week() -> &'static str {
    "STRFTIME('%d', DATE(todos.due, 'weekday 0', '-6 days')) || ' ' || 
    SUBSTR('JanFebMarAprMayJunJulAugSepOctNovDec', 1 + 3 * (STRFTIME('%m', DATE(todos.due, 'weekday 0', '-6 days')) - 1), 3)"
}

fn for_week_range() -> &'static str {
    "DATE('now', '-35 days', 'weekday 0', '-6 days') AND DATE('now', 'weekday 0')"
}
 
fn for_month() -> &'static str {
    "SUBSTR('JanFebMarAprMayJunJulAugSepOctNovDec', 1 + 3 * (STRFTIME('%m', todos.due) - 1), 3) || ' ' || STRFTIME('%Y', todos.due)"
}

fn for_month_range() -> &'static str {
    "DATE('now', '-6 months', 'start of month') AND DATE('now', 'start of month', '+1 month', '-1 day')"
}
