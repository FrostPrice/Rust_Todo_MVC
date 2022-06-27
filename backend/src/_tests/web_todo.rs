use super::todo_rest_filters;
use crate::{
    model::{init_db, Todo, TodoMac, TodoStatus},
    security::utx_from_token,
    web::handle_rejection,
};
use anyhow::{Context, Result};
use serde::Deserialize;
use serde_json::{from_str, from_value, json, Value};
use std::{str::from_utf8, sync::Arc};
use warp::{
    ext,
    hyper::{body::Bytes, Response},
    Filter,
};

#[tokio::test]
async fn web_todo_list() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let todo_apis = todo_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "123") // .user_info_b64.sign_b64
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

#[tokio::test]
async fn web_todo_get_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let todo_apis = todo_rest_filters("api", db).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("GET")
        .header("X-Auth-Token", "123")
        .path("/api/todos/100")
        .reply(&todo_apis)
        .await;

    // -- CHECK - Status
    assert_eq!(200, resp.status(), "http status");

    // Extract respose .data
    let todo: Todo = extract_body_data(resp)?;

    // -- CHECK - .data (todo)
    assert_eq!(100, todo.id);
    assert_eq!("todo 100", todo.title);
    assert_eq!(TodoStatus::Close, todo.status);

    Ok(())
}

#[tokio::test]

async fn web_todo_create_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let todo_apis = todo_rest_filters("api", db.clone()).recover(handle_rejection);

    // New todo fixture
    const TITLE: &str = "test - web_todo_create_ok";
    let body = json!({ "title": TITLE });

    // -- ACTION
    let resp = warp::test::request()
        .method("POST")
        .header("X-Auth-Token", "123")
        .path("/api/todos")
        .json(&body)
        .reply(&todo_apis)
        .await;

    // -- CHECK - Status
    assert_eq!(200, resp.status(), "http status");

    // Extract respose .data
    let todo: Todo = extract_body_data(resp)?;

    // -- CHECK - .data (todo)
    assert!(todo.id >= 1000, "todo.id should be >= to 1000");
    assert_eq!(TITLE, todo.title);
    assert_eq!(TodoStatus::Open, todo.status);

    Ok(())
}

#[tokio::test]
async fn web_todo_update_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let todo_apis = todo_rest_filters("api", db.clone()).recover(handle_rejection);

    // Updated todo
    const TITLE: &str = "test - todo 100 updated";
    let body = json!({ "title": TITLE , "status": "Open"});

    // -- ACTION
    let resp = warp::test::request()
        .method("PATCH")
        .header("X-Auth-Token", "123")
        .path("/api/todos/100")
        .json(&body)
        .reply(&todo_apis)
        .await;

    // -- CHECK - Status
    assert_eq!(200, resp.status(), "http status");

    // Extract respose .data
    let todo: Todo = extract_body_data(resp)?;

    // -- CHECK - .data (todo)
    assert_eq!(100, todo.id, "todo.id");
    assert_eq!(TITLE, todo.title);
    assert_eq!(TodoStatus::Open, todo.status);

    Ok(())
}

#[tokio::test]
async fn web_todo_delete_ok() -> Result<()> {
    // -- FIXTURE
    let db = init_db().await?;
    let db = Arc::new(db);
    let todo_apis = todo_rest_filters("api", db.clone()).recover(handle_rejection);

    // -- ACTION
    let resp = warp::test::request()
        .method("DELETE")
        .header("X-Auth-Token", "123")
        .path("/api/todos/100")
        .reply(&todo_apis)
        .await;

    // -- CHECK - Status
    assert_eq!(200, resp.status(), "http status");

    // Extract respose .data
    let todo: Todo = extract_body_data(resp)?;

    // -- CHECK - .data (todo)
    assert_eq!(100, todo.id);
    assert_eq!("todo 100", todo.title);
    assert_eq!(TodoStatus::Close, todo.status);

    // -- CHECK - list .len() should be 1
    let utx = utx_from_token(&db, "123").await?;
    let todos = TodoMac::list(&db, &utx).await?;
    assert_eq!(1, todos.len(), "todos length");
    assert_eq!(101, todos[0].id, "Todo remaining should be 101");

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
