use askama::Template;
use axum::{
    Extension, Form, Json,
    extract::{Path, Query, State},
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};

use super::CurrentUser;
use super::{BaseTemplate, HtmlTemplate};
use crate::{
    models::Insight,
    repo::insigts::{
        create_insight, delete_insight, get_efforts, get_efforts_by_tags, get_insight,
        get_insights, update_insight,
    },
};

#[derive(Template)]
#[template(path = "insights.html")]
struct InsightsTemplate {
    base: BaseTemplate,
    insights: Vec<Insight>,
    select_id: Option<i64>,
}

pub async fn insights_page(
    id: Option<Path<i64>>,
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
) -> Response {
    let select_id = if let Some(Path(id)) = id {
        Some(id)
    } else {
        None
    };

    let insights = match get_insights(&pool, user.user_id).await {
        Ok(insights) => insights,
        Err(err) => {
            eprintln!("Error from get_insights. {:?}", err);
            return Html("<p>Error in getting insights. Please contact admin</p>".to_string())
                .into_response();
        }
    };

    let template = InsightsTemplate {
        base: BaseTemplate::new(headers).await,
        insights,
        select_id,
    };

    HtmlTemplate(template).into_response()
}

#[derive(Deserialize)]
pub struct EffortQuery {
    pub period: String,
}

#[derive(Serialize)]
pub struct InsightData {
    pub chart_type: String,
    pub labels: Vec<String>,
    pub data_sets: HashMap<String, Vec<f64>>,
}

pub async fn get_insight_data(
    Extension(user): Extension<CurrentUser>,
    Query(params): Query<EffortQuery>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Json<InsightData> {
    let default_chart_type = "line".to_string();
    let mut labels = Vec::new();
    let mut data_sets: HashMap<String, Vec<f64>> = HashMap::new();

    let insight = match get_insight(&pool, user.user_id, id).await {
        Ok(insight) => insight,
        Err(err) => {
            eprintln!("Error from get_insight. {:?}", err);
            return Json(InsightData {
                chart_type: default_chart_type,
                labels,
                data_sets,
            });
        }
    };

    let tags = match insight.tags {
        Some(tags) => tags.trim().to_string(),
        None => "".to_string(),
    };

    let period = params.period.as_str();
    if insight.metric == "effort" {
        let result = if tags.is_empty() {
            get_efforts(&pool, user.user_id, period).await
        } else {
            get_efforts_by_tags(&pool, user.user_id, period, &tags).await
        };

        match result {
            Ok(values) => {
                labels = values.0;
                data_sets = values.1;
            }
            Err(err) => {
                eprintln!(
                    "Error: unable to get data values for insight: {}. {:?}",
                    insight.name, err
                );
            }
        }
    } else {
        eprintln!(
            "Error: Unknown metric {} for insight: {}",
            insight.metric, insight.name
        );
    }

    Json(InsightData {
        chart_type: insight.chart_type,
        labels,
        data_sets,
    })
}

#[derive(Template)]
#[template(path = "insight_edit.html")]
pub struct InsightFormTemplate {
    base: BaseTemplate,
    insight: InsightForm,
    is_edit: bool,
    error: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct InsightForm {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub metric: String,
    pub chart_type: String,
    pub tags: Option<String>,
}

pub async fn new_insight(headers: HeaderMap) -> Response {
    let insight = InsightForm {
        id: None,
        name: String::new(),
        description: None,
        metric: String::new(),
        chart_type: String::new(),
        tags: None,
    };

    let template = InsightFormTemplate {
        base: BaseTemplate::new(headers).await,
        insight,
        is_edit: false,
        error: None,
    };

    HtmlTemplate(template).into_response()
}

pub async fn edit_insight(
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(insight_id): Path<i64>,
) -> Response {
    let insight = match get_insight(&pool, user.user_id, insight_id).await {
        Ok(insight) => insight,
        Err(err) => {
            eprintln!("Error in getting insight for ID {}. {:?}", insight_id, err);
            return Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body("Insight not found".into())
                .unwrap();
        }
    };

    let insight = InsightForm {
        id: Some(insight.id),
        name: insight.name,
        description: insight.description,
        metric: insight.metric,
        chart_type: insight.chart_type,
        tags: insight.tags,
    };

    let template = InsightFormTemplate {
        base: BaseTemplate::new(headers).await,
        insight,
        is_edit: true,
        error: None,
    };

    HtmlTemplate(template).into_response()
}

pub async fn save_insight(
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Form(form): Form<InsightForm>,
) -> Response {
    let copy = form.clone();

    let insight = Insight {
        id: copy.id.unwrap_or(-1),
        user_id: user.user_id,
        name: copy.name,
        description: copy.description,
        metric: copy.metric,
        chart_type: copy.chart_type,
        tags: copy.tags,
        periods: None,
    };

    let result = match copy.id {
        Some(_) => update_insight(&pool, &insight).await,
        None => create_insight(&pool, &insight).await,
    };

    let insight = match result {
        Ok(insight) => insight,
        Err(err) => {
            let s = &err.to_string();
            eprintln!("Error from create_insight. {:?}", s);
            let template = InsightFormTemplate {
                base: BaseTemplate::new(headers).await,
                insight: form,
                is_edit: false,
                error: Some(s.to_string()),
            };

            return HtmlTemplate(template).into_response();
        }
    };

    let path = format!(
        r#"<script>window.location.href='/insights/{}';</script>"#,
        insight.id
    );
    Html(path).into_response()
}

pub async fn delete_insight_h(
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Response {
    match delete_insight(&pool, user.user_id, id).await {
        Ok(_) => (),
        Err(e) => eprintln!("Error deleting insight {}. {:?}", id, e), // ✅ Corrected variable
    }

    let redirect_script = r#"<script>window.location.href='/insights';</script>"#;
    Html(redirect_script).into_response()
}
