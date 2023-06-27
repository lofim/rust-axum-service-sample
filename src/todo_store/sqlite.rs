use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;

use crate::error::Error;

use crate::model::{Todo, TodoInput};
use crate::use_cases::TodoOutputPort;

pub struct SqliteTodoStore {
    pool: SqlitePool,
}

impl SqliteTodoStore {
    pub fn new(pool: SqlitePool) -> Self {
        SqliteTodoStore { pool }
    }
}

#[async_trait]
impl TodoOutputPort for SqliteTodoStore {
    async fn list_todos(&self) -> Result<Vec<Todo>, Error> {
        let result = sqlx::query_as::<_, Todo>("select id, text, state from todos")
            .fetch_all(&self.pool)
            .await?;

        Ok(result)
    }

    async fn get_todo(&self, id: u32) -> Result<Todo, Error> {
        let result = sqlx::query_as::<_, Todo>("select id, text, state from todos where id = ?")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| Error::from_with_context(e, "todo".to_owned(), id))?;

        Ok(result)
    }

    async fn create_todo(&self, todo: TodoInput) -> Result<Todo, Error> {
        let result = sqlx::query_as::<_, Todo>(
            "insert into todos (text, state) values (? ,?) returning id, text, state",
        )
        .bind(todo.text)
        .bind(todo.state)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    async fn update_todo(&self, id: u32, todo: TodoInput) -> Result<Todo, Error> {
        let result = sqlx::query_as::<_, Todo>(
            "update todos set text = ?, state = ? where id = ? returning id, text, state",
        )
        .bind(todo.text)
        .bind(todo.state)
        .bind(id)
        .fetch_one(&self.pool)
        // TODO: find a better way how to do this
        // We want to pass the context to our error but ideally without boilerplate
        .await
        .map_err(|e| Error::from_with_context(e, "todo".to_owned(), id))?;

        Ok(result)
    }

    async fn delete_todo(&self, id: u32) -> Result<(), Error> {
        sqlx::query("delete from todos where id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
