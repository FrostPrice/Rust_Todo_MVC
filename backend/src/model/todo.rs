use sqlb::HasFields;
use sqlx::query_as;

use super::db::Db;
use crate::{model, security::UserCtx};

// region: Todo Types
#[derive(sqlx::FromRow, Debug, Clone)]
pub struct Todo {
    pub id: i64,
    pub cid: i64, // Creator ID
    pub title: String,
    pub status: TodoStatus,
}

#[derive(sqlb::Fields, Default, Clone)]
pub struct TodoPatch {
    pub title: Option<String>,
    pub status: Option<TodoStatus>,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Eq)]
#[sqlx(type_name = "todo_status_enum")]
#[sqlx(rename_all = "lowercase")]
pub enum TodoStatus {
    Open,
    Close,
}
sqlb::bindable!(TodoStatus);
// endregion: Todo Types

// region: TodoMac
pub struct TodoMac;

impl TodoMac {
    pub async fn create(db: &Db, utx: &UserCtx, data: TodoPatch) -> Result<Todo, model::Error> {
        // Hardcoding it
        // let sql = "INSERT INTO todo (cid, title) VALUES ($1, $2) returning id, cid, title, status";
        // let query = sqlx::query_as::<_, Todo>(&sql)
        //     .bind(123 as i64) // FIXME -- Should come from user context
        //     .bind(data.title.unwrap_or_else(|| "untitled".to_string()));

        // Using Sql Builder
        let mut fields = data.fields();
        fields.push(("cid", 123).into());
        let sb = sqlb::insert()
            .table("todo")
            .data(fields)
            .returning(&["id", "cid", "title", "status"]);

        let todo = sb.fetch_one(db).await?;

        Ok(todo)
    }

    pub async fn list(db: &Db, _utx: &UserCtx) -> Result<Vec<Todo>, model::Error> {
        // Hardcoding it
        // let sql = "SELECT id, cid, title, status FROM todo ORDER BY id DESC";

        // Using Sql Builder
        let sb = sqlb::select()
            .table("todo")
            .columns(&["id", "cid", "title", "status"])
            .order_by("!id");

        // Build the sqlx-query (For the Hardcoding it)
        // let query = sqlx::query_as(&sql);

        // Execute the query
        let todos = sb.fetch_all(db).await?;

        Ok(todos)
    }
}
// endregion: TodoMac

#[cfg(test)]
#[path = "../_tests/model_todo.rs"]
mod tests;
