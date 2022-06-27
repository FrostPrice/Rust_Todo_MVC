use super::todo_rest_filters;
use crate::model::{init_db, Todo, TodoStatus};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, Value};
use std::{str::from_utf8, sync::Arc};
use warp::hyper::{body::Bytes, Response};

#[tokio::test]
async fn web_todo_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let todo_apis = todo_rest_filters("api", db.clone());

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .path("/api/todos")
        .reply(&todo_apis)
        .await;

    // -- CHECK
    assert_eq!(200, resp.status(), "http status");

    // Extract response .data
    let todos: Vec<Todo> = extract_body_data(resp)?;

    // -- CHECK - Todos
    assert_eq!(2, todos.len(), "Number of Todos");
    assert_eq!(101, todos[0].id);
    assert_eq!("todo 101", todos[0].title);
    assert_eq!(TodoStatus::Open, todos[0].status);

    Ok(())
}

// region: Web Test Utils
fn extract_body_data<D>(resp: Response<Bytes>) -> Result<D>
where
    for<'de> D: Deserialize<'de>,
{
    // Parse the body as serde_json::Value
    let body = from_utf8(resp.body())?;
    let mut body: Value = from_str(body)
        .with_context(|| format!("Cannot parse resp.body to JSON. resp.body: '{}'", body))?;

    // Extract the Data
    let data = body["data"].take();

    // Deserialize the Data to D
    let data: D = from_value(data)?;

    Ok(data)
}
// endregion: Web Test Utils