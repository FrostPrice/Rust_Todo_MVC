use thiserror::Error as ThisError;

mod db;
mod todo;
pub use todo::{Todo, TodoMac, TodoPatch, TodoStatus};

// re-export
pub use db::init_db;
pub use db::Db;

// region: Error
#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Entity Not Found - {0}[{1}]")]
    EntityNotFound(&'static str, String),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error(transparent)]
    IOError(#[from] std::io::Error),
}
// endregion: Error
