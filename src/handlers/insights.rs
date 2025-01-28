use std::{collections::HashMap, sync::Arc};
use askama::Template;
use sqlx::SqlitePool;
use serde::{Deserialize, Serialize};
use axum::{
    Json,
    Extension,
    Form,
    extract::{Path, State, Query}, 
    http::HeaderMap,
    response::{Html, IntoResponse, Response}
};

use crate::{
    models::Insight, 
    repo::insigts::{
        create_insight, 
        get_efforts, 
        get_efforts_by_tags, 
        get_insight, 
        get_insights
    }
};
use super::CurrentUser;
use super::{BaseTemplate, HtmlTemplate};

#[derive(Template)]
#[template(path = "insights.html")]
struct InsightsTemplate {
    base: BaseTemplate,
    insights: Vec<Insight>,
}

pub async fn insights_page(
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
) -> Response {
    let insights = match get_insights(&pool, user.user_id).await {
        Ok(insights) => insights,
        Err(err) => {
            eprintln!("Error from get_insights. {:?}", err);
            return  Html("<p>Error in getting insights. Please contact admin</p>".to_string())
                .into_response();
        }
    };

    let template = InsightsTemplate {
        base: BaseTemplate::new(headers).await,
        insights,
    };

    HtmlTemplate(template).into_response()
}


#[derive(Deserialize)]
pub struct EffortQuery {
    pub period: String, 
}

#[derive(Serialize)]
pub struct InsightData {
    pub labels: Vec<String>,
    pub data_sets: HashMap<String, Vec<f64>>,
}

pub async fn get_insight_data(
    Extension(user): Extension<CurrentUser>,
    Query(params): Query<EffortQuery>,
    State(pool): State<Arc<SqlitePool>>,
    Path(id): Path<i64>,
) -> Json<InsightData> {

    let mut labels = Vec::new();
    let mut data_sets: HashMap<String, Vec<f64>> = HashMap::new();

    let insight = match get_insight(&pool, user.user_id, id).await {
        Ok(insight) => insight,
        Err(err) => {
            eprintln!("Error from get_insight. {:?}", err);
            return Json(InsightData { labels, data_sets });
        }
    };

    let period = params.period.as_str();
    if insight.metric == "effort" {
        let result = match insight.tags {
            Some(tags) => get_efforts_by_tags(&pool, user.user_id, period, &tags).await,
            None => get_efforts(&pool, user.user_id, period).await
        };

        match result {
            Ok(values) => {
                labels = values.0;
                data_sets = values.1;
            }
            Err(err) => {
                eprintln!("Error: unable to get data values for insight: {}. {:?}", insight.name, err);
            }
        }
    } else {
        eprintln!("Error: unable to retrive any insight data for insight: {:?}", insight);
    }
    
    Json(InsightData { labels, data_sets })
}

// Template for rendering Create/Edit page
#[derive(Template)]
#[template(path = "insight_edit.html")]
pub struct InsightFormTemplate {
    base: BaseTemplate,
    insight: Insight,
    is_edit: bool,
    error: Option<String>,
}

pub async fn new_insight(
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
) -> Response {

    let insight = Insight {
        id: -1,
        user_id: user.user_id, 
        name: String::new(),
        description: None,
        metric: String::new(),
        chart_type: String::new(),
        tags: None,
        periods: None,
    };

    let template = InsightFormTemplate { 
        base: BaseTemplate::new(headers).await,
        insight, 
        is_edit: false,
        error: None,
    };

    HtmlTemplate(template).into_response()
}

#[derive(Debug, Deserialize)]
pub struct InsightForm {
    pub name: String,
    pub description: Option<String>,
    pub metric: String,
    pub chart_type: String,
    pub tags: Option<String>, 
}

pub async fn save_insight(
    headers: HeaderMap,
    Extension(user): Extension<CurrentUser>,
    State(pool): State<Arc<SqlitePool>>,
    Form(form): Form<InsightForm>,
) -> Response {

    let insight = Insight {
        id: -1,
        user_id: user.user_id,
        name: form.name,
        description: form.description,
        metric: form.metric,
        chart_type: form.chart_type,
        tags: form.tags,
        periods: None,
    };

     if let Some(tags) = &insight.tags {
         if tags.is_empty() {
        let template = InsightFormTemplate { 
                 base: BaseTemplate::new(headers).await,
                 insight, 
                 is_edit: false,
                 error: Some("tags cannot be empty".to_string()),
             };

             return HtmlTemplate(template).into_response()
         }

    }


    let insight = match create_insight(&pool, &insight).await {
        Ok(insight) => insight,
        Err(err) => {
             let s = &err.to_string();
             eprintln!("Error from create_insight. {:?}", s);
             let template = InsightFormTemplate { 
                 base: BaseTemplate::new(headers).await,
                 insight, 
                 is_edit: false,
                 error: Some(s.to_string()),
             };

             return HtmlTemplate(template).into_response()
        }
    };

    println!("insight created with id: {:?}", insight.id);
    Html(r#"<script>window.location.href='/insights';</script>"#).into_response()
}
