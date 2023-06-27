use async_trait::async_trait;
use std::sync::Arc;

use crate::error::Result;
use crate::model::{Todo, TodoInput};

// This is rust specific thing. We need to be able to send the stuff across threads
// TODO: figure out if there's an alternative way how to strucutre the app
pub type TodoInputPortArc = Arc<dyn TodoInputPort + Send + Sync>;
pub type TodoOutputPortArc = Arc<dyn TodoOutputPort + Send + Sync>;

// This is the user case (input port defines invokable logic)
#[async_trait]
pub trait TodoInputPort {
    async fn list_todos(&self) -> Result<Vec<Todo>>;
    async fn get_todo(&self, id: u32) -> Result<Todo>;
    async fn create_todo(&self, todo: TodoInput) -> Result<Todo>;
    async fn update_todo(&self, id: u32, todo: TodoInput) -> Result<Todo>;
    async fn delete_todo(&self, id: u32) -> Result<()>;
}

// This is sotre (output port defines dependency of the user case)
#[async_trait]
pub trait TodoOutputPort {
    async fn list_todos(&self) -> Result<Vec<Todo>>;
    async fn get_todo(&self, id: u32) -> Result<Todo>;
    async fn create_todo(&self, todo: TodoInput) -> Result<Todo>;
    async fn update_todo(&self, id: u32, todo: TodoInput) -> Result<Todo>;
    async fn delete_todo(&self, id: u32) -> Result<()>;
}

pub struct TodoService {
    todo_store: TodoOutputPortArc,
}

impl TodoService {
    pub fn new(todo_store: TodoOutputPortArc) -> Self {
        Self { todo_store }
    }
}

// There's not much logic needed as the sample demostrated a CRUD app
// This would usually hold the application specific (use case logic)
#[async_trait]
impl TodoInputPort for TodoService {
    async fn list_todos(&self) -> Result<Vec<Todo>> {
        Ok(self.todo_store.list_todos().await?)
    }

    async fn get_todo(&self, id: u32) -> Result<Todo> {
        Ok(self.todo_store.get_todo(id).await?)
    }

    async fn create_todo(&self, todo: TodoInput) -> Result<Todo> {
        Ok(self.todo_store.create_todo(todo).await?)
    }

    async fn update_todo(&self, id: u32, todo: TodoInput) -> Result<Todo> {
        Ok(self.todo_store.update_todo(id, todo).await?)
    }

    async fn delete_todo(&self, id: u32) -> Result<()> {
        Ok(self.todo_store.delete_todo(id).await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::Error, model::TodoState, todo_store::inmemory::InMemoryTodoStore};

    use super::*;

    #[tokio::test]
    async fn list_todos_should_return_all_items() {
        let todo_store = InMemoryTodoStore::new();
        let todo_service = TodoService::new(Arc::new(todo_store));

        let todo1 = TodoInput {
            text: "First Test Item".to_owned(),
            state: TodoState::Opened,
        };
        let todo2 = TodoInput {
            text: "Second Test Item".to_owned(),
            state: TodoState::Closed,
        };

        todo_service.create_todo(todo1).await.unwrap();
        todo_service.create_todo(todo2).await.unwrap();

        let result = todo_service.list_todos().await.unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn create_todo_should_add_new_item_into_store() {
        let todo_store = InMemoryTodoStore::new();
        let todo_service = TodoService::new(Arc::new(todo_store));

        let todo = TodoInput {
            text: "First Test Item".to_owned(),
            state: TodoState::Opened,
        };
        let inserted_todo = todo_service.create_todo(todo).await.unwrap();
        let result = todo_service.list_todos().await.unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result.first().unwrap().to_owned(), inserted_todo);
    }

    #[tokio::test]
    async fn get_todo_should_return_existing_item() {
        let todo_store = InMemoryTodoStore::new();
        let todo_service = TodoService::new(Arc::new(todo_store));

        let todo = TodoInput {
            text: "First Test Item".to_owned(),
            state: TodoState::Opened,
        };
        let inserted_todo = todo_service.create_todo(todo).await.unwrap();
        let result = todo_service.get_todo(inserted_todo.id).await.unwrap();

        assert_eq!(result, inserted_todo);
    }

    #[tokio::test]
    async fn get_todo_should_return_error_on_missing_item() {
        let todo_store = InMemoryTodoStore::new();
        let todo_service = TodoService::new(Arc::new(todo_store));

        match todo_service.get_todo(999).await {
            Err(Error::ResourceNotFound { name, id }) => {
                assert_eq!(name, "todo".to_owned());
                assert_eq!(id, 999);
            }
            _ => assert!(
                false,
                "The test should not hit this branch. We expect the action to return an error."
            ),
        }
    }
}
