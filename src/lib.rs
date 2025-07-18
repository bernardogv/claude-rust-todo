use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: Uuid,
    pub text: String,
    pub completed: bool,
    pub priority: Priority,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct TodoList {
    todos: Vec<Todo>,
}

impl TodoList {
    pub fn new() -> Self {
        // TODO: Implement this
        todo!()
    }

    pub fn add_todo(&mut self, text: String, priority: Priority) -> Result<&Todo, String> {
        // TODO: Implement this method
        // Should validate text is not empty
        // Should create a new Todo with unique ID
        // Should add it to the list and return a reference
        todo!()
    }

    pub fn remove_todo(&mut self, id: Uuid) -> Result<Todo, String> {
        // TODO: Implement this method
        // Should find and remove the todo by ID
        // Should return error if not found
        todo!()
    }

    pub fn toggle_todo(&mut self, id: Uuid) -> Result<&Todo, String> {
        // TODO: Implement this method
        // Should toggle the completed status
        // Should update the updated_at timestamp
        todo!()
    }

    pub fn get_all_todos(&self) -> &[Todo] {
        // TODO: Implement this
        todo!()
    }

    pub fn get_todo_by_id(&self, id: Uuid) -> Option<&Todo> {
        // TODO: Implement this
        todo!()
    }

    pub fn get_completed_todos(&self) -> Vec<&Todo> {
        // TODO: Implement this
        todo!()
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement persistence
        todo!()
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Implement loading from file
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_todo_list() {
        let todos = TodoList::new();
        assert!(todos.get_all_todos().is_empty());
    }

    // TODO: Add more tests
}
