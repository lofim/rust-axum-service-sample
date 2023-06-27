use crate::error::{Error, Result};
use crate::model::{Todo, TodoInput};
use crate::use_cases::TodoOutputPort;
use async_trait::async_trait;
use std::sync::Mutex;

pub struct InMemoryTodoStore {
    todo_store: Mutex<Vec<Todo>>,
}

impl InMemoryTodoStore {
    #[allow(dead_code)]
    // Not sure why this is reported as not used while it's used in tests
    pub fn new() -> Self {
        Self {
            todo_store: Mutex::new(Vec::new()),
        }
    }
}

#[async_trait]
impl TodoOutputPort for InMemoryTodoStore {
    async fn list_todos(&self) -> Result<Vec<Todo>> {
        Ok(self.todo_store.lock().unwrap().to_vec())
    }

    async fn get_todo(&self, id: u32) -> Result<Todo> {
        let list = self.todo_store.lock().unwrap();
        let result =
            list.clone()
                .into_iter()
                .find(|todo| todo.id == id)
                .ok_or(Error::ResourceNotFound {
                    name: "todo".to_owned(),
                    id,
                })?;

        Ok(result)
    }

    async fn create_todo(&self, todo: TodoInput) -> Result<Todo> {
        let mut locked_store = self.todo_store.lock().unwrap();
        let max_id = locked_store.iter().map(|t| t.id).max().unwrap_or(1);
        let new_todo = Todo {
            id: max_id + 1,
            state: todo.state,
            text: todo.text,
        };

        locked_store.push(new_todo.clone());
        Ok(new_todo)
    }

    async fn update_todo(&self, id: u32, todo: TodoInput) -> Result<Todo> {
        let mut locked_store = self.todo_store.lock().unwrap();
        let todo_index = locked_store.iter().position(|todo| todo.id == id).unwrap();

        locked_store[todo_index].state = todo.state;
        locked_store[todo_index].text = todo.text;

        Ok(locked_store[todo_index].clone())
    }

    async fn delete_todo(&self, id: u32) -> Result<()> {
        let mut locked_store = self.todo_store.lock().unwrap();
        let todo_index = locked_store.iter().position(|todo| todo.id == id).unwrap();

        locked_store.remove(todo_index);
        Ok(())
    }
}
