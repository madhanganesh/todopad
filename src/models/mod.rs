use chrono::{NaiveDate, Utc};
use chrono_tz::Tz;
use regex::Regex;
use serde::Serialize;
use sqlx::FromRow;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Todo {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
    pub due: Option<NaiveDate>,
    pub effort: f64,
    pub completed: bool,
    pub notes: Option<String>,
}

impl Todo {
    pub fn notes_or_empty(&self) -> &str {
        self.notes.as_deref().unwrap_or("")
    }

    pub fn relative_due(&self, timezone: &str) -> String {
        let due = self.due.unwrap();
        let tz: Tz = timezone.parse().unwrap_or(chrono_tz::UTC);
        let now_in_tz = Utc::now().with_timezone(&tz);
        let today = now_in_tz.date_naive();
        let difference = (due - today).num_days();

        match difference {
            0 => "today".to_string(),
            1 => "tomorrow".to_string(),
            -1 => "yesterday".to_string(),
            _ => due.format("%d %b").to_string(),
        }
    }

    pub fn extract_links(&self) -> Vec<String> {
        let url_regex = Regex::new(r"https?://[^\s]+").unwrap();
        url_regex
            .find_iter(self.notes.as_deref().unwrap_or(""))
            .map(|mat| mat.as_str().to_string())
            .take(3)
            .collect()
    }
}

#[derive(Serialize, FromRow)]
pub struct Tag {
    pub user_id: i64,
    pub todo_id: i64,
    pub tag: String,
}

#[derive(Debug, Serialize, FromRow)]
pub struct Insight {
    pub id: i64,
    pub user_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub metric: String,
    pub chart_type: String,
    pub tags: Option<String>,
    pub periods: Option<String>,
}
