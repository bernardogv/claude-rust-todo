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
        Self { todos: Vec::new() }
    }

    pub fn add_todo(&mut self, text: String, priority: Priority) -> Result<&Todo, String> {
        if text.trim().is_empty() {
            return Err("Todo text cannot be empty".to_string());
        }

        let now = Utc::now();
        let todo = Todo {
            id: Uuid::new_v4(),
            text,
            completed: false,
            priority,
            created_at: now,
            updated_at: now,
        };

        self.todos.push(todo);
        Ok(self.todos.last().expect("Todo just added should exist"))
    }

    pub fn remove_todo(&mut self, id: Uuid) -> Result<Todo, String> {
        let position = self
            .todos
            .iter()
            .position(|todo| todo.id == id)
            .ok_or_else(|| format!("Todo with id {} not found", id))?;

        Ok(self.todos.remove(position))
    }

    pub fn toggle_todo(&mut self, id: Uuid) -> Result<&Todo, String> {
        let todo = self
            .todos
            .iter_mut()
            .find(|todo| todo.id == id)
            .ok_or_else(|| format!("Todo with id {} not found", id))?;

        todo.completed = !todo.completed;
        todo.updated_at = Utc::now();

        Ok(todo)
    }

    pub fn get_all_todos(&self) -> &[Todo] {
        &self.todos
    }

    pub fn get_todo_by_id(&self, id: Uuid) -> Option<&Todo> {
        self.todos.iter().find(|todo| todo.id == id)
    }

    pub fn get_completed_todos(&self) -> Vec<&Todo> {
        self.todos.iter().filter(|todo| todo.completed).collect()
    }

    pub fn save_to_file(&self, _path: &str) -> Result<(), Box<dyn std::error::Error>> {
        // TODO: Implement persistence
        Err("save_to_file not yet implemented".into())
    }

    pub fn load_from_file(_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // TODO: Implement loading from file
        Err("load_from_file not yet implemented".into())
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

    #[test]
    fn test_add_todo_success() {
        let mut todos = TodoList::new();
        let result = todos.add_todo("Test todo".to_string(), Priority::Medium);

        assert!(result.is_ok());
        let todo = result.unwrap();
        assert_eq!(todo.text, "Test todo");
        assert_eq!(todo.priority, Priority::Medium);
        assert!(!todo.completed);
        assert_eq!(todos.get_all_todos().len(), 1);
    }

    #[test]
    fn test_add_todo_empty_text() {
        let mut todos = TodoList::new();
        let result = todos.add_todo("".to_string(), Priority::Low);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Todo text cannot be empty");
        assert!(todos.get_all_todos().is_empty());
    }

    #[test]
    fn test_add_todo_whitespace_only() {
        let mut todos = TodoList::new();
        let result = todos.add_todo("   ".to_string(), Priority::High);

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Todo text cannot be empty");
        assert!(todos.get_all_todos().is_empty());
    }

    #[test]
    fn test_remove_todo_success() {
        let mut todos = TodoList::new();
        let todo = todos
            .add_todo("Test todo".to_string(), Priority::Low)
            .unwrap();
        let todo_id = todo.id;

        let removed = todos.remove_todo(todo_id);
        assert!(removed.is_ok());
        assert_eq!(removed.unwrap().text, "Test todo");
        assert!(todos.get_all_todos().is_empty());
    }

    #[test]
    fn test_remove_todo_not_found() {
        let mut todos = TodoList::new();
        let fake_id = Uuid::new_v4();

        let result = todos.remove_todo(fake_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_toggle_todo_success() {
        let mut todos = TodoList::new();
        let todo = todos
            .add_todo("Test todo".to_string(), Priority::Medium)
            .unwrap();
        let todo_id = todo.id;
        let original_updated_at = todo.updated_at;

        // Toggle to completed
        let result = todos.toggle_todo(todo_id);
        assert!(result.is_ok());
        let toggled_todo = result.unwrap();
        assert!(toggled_todo.completed);
        assert!(toggled_todo.updated_at > original_updated_at);
        let first_toggle_time = toggled_todo.updated_at;

        // Toggle back to incomplete
        let result2 = todos.toggle_todo(todo_id);
        assert!(result2.is_ok());
        let toggled_todo2 = result2.unwrap();
        assert!(!toggled_todo2.completed);
        assert!(toggled_todo2.updated_at > first_toggle_time);
    }

    #[test]
    fn test_toggle_todo_not_found() {
        let mut todos = TodoList::new();
        let fake_id = Uuid::new_v4();

        let result = todos.toggle_todo(fake_id);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("not found"));
    }

    #[test]
    fn test_get_todo_by_id_success() {
        let mut todos = TodoList::new();
        let todo = todos
            .add_todo("Test todo".to_string(), Priority::High)
            .unwrap();
        let todo_id = todo.id;

        let found = todos.get_todo_by_id(todo_id);
        assert!(found.is_some());
        assert_eq!(found.unwrap().text, "Test todo");
    }

    #[test]
    fn test_get_todo_by_id_not_found() {
        let todos = TodoList::new();
        let fake_id = Uuid::new_v4();

        let found = todos.get_todo_by_id(fake_id);
        assert!(found.is_none());
    }

    #[test]
    fn test_get_completed_todos() {
        let mut todos = TodoList::new();

        // Add multiple todos
        let todo1_id = todos
            .add_todo("Todo 1".to_string(), Priority::Low)
            .unwrap()
            .id;
        let _todo2_id = todos
            .add_todo("Todo 2".to_string(), Priority::Medium)
            .unwrap()
            .id;
        let todo3_id = todos
            .add_todo("Todo 3".to_string(), Priority::High)
            .unwrap()
            .id;

        // Mark some as completed
        todos.toggle_todo(todo1_id).unwrap();
        todos.toggle_todo(todo3_id).unwrap();

        let completed = todos.get_completed_todos();
        assert_eq!(completed.len(), 2);
        assert!(completed.iter().any(|t| t.text == "Todo 1"));
        assert!(completed.iter().any(|t| t.text == "Todo 3"));
        assert!(!completed.iter().any(|t| t.text == "Todo 2"));
    }

    #[test]
    fn test_get_completed_todos_empty() {
        let todos = TodoList::new();
        let completed = todos.get_completed_todos();
        assert!(completed.is_empty());
    }

    #[test]
    fn test_get_all_todos_multiple() {
        let mut todos = TodoList::new();

        todos.add_todo("Todo 1".to_string(), Priority::Low).unwrap();
        todos
            .add_todo("Todo 2".to_string(), Priority::Medium)
            .unwrap();
        todos
            .add_todo("Todo 3".to_string(), Priority::High)
            .unwrap();

        let all_todos = todos.get_all_todos();
        assert_eq!(all_todos.len(), 3);
        assert_eq!(all_todos[0].text, "Todo 1");
        assert_eq!(all_todos[1].text, "Todo 2");
        assert_eq!(all_todos[2].text, "Todo 3");
    }

    #[test]
    fn test_todo_timestamps() {
        let mut todos = TodoList::new();
        let todo = todos
            .add_todo("Test todo".to_string(), Priority::Medium)
            .unwrap();
        let todo_id = todo.id;
        let created_at = todo.created_at;
        let initial_updated_at = todo.updated_at;

        assert_eq!(created_at, initial_updated_at);

        // Toggle and check updated_at changes
        let updated_todo = todos.toggle_todo(todo_id).unwrap();
        assert!(updated_todo.updated_at > created_at);
    }

    #[test]
    fn test_unique_ids() {
        let mut todos = TodoList::new();
        let todo1_id = todos
            .add_todo("Todo 1".to_string(), Priority::Low)
            .unwrap()
            .id;
        let todo2_id = todos
            .add_todo("Todo 2".to_string(), Priority::Medium)
            .unwrap()
            .id;

        assert_ne!(todo1_id, todo2_id);
    }
}
