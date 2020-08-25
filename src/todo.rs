use crate::error::TodoResult;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use ansi_term::Style;

#[derive(Serialize, Deserialize)]
pub struct TodoItem {
    pub id: u32,
    pub title: String,
    #[serde(default)]
    pub text: String,
    pub done: bool,
    #[serde(default)]
    pub priority: i16,
    #[serde(default)]
    pub dependencies: BTreeSet<u32>,
    #[serde(default)]
    pub parent: Option<u32>,
}

impl TodoItem {
    pub fn format_depends(&self, list: &TodoList, ansi: bool) -> String {
        use std::fmt::Write;
        let mut result = String::new();
        for (ii, e) in self.dependencies.iter().enumerate() {
            if ansi && list.by_id(*e).unwrap().done {
                write!(result, "{}", Style::new().strikethrough().paint(&format!("#{}", e))).unwrap();
            } else {
                write!(result, "#{}", e).unwrap();
            }
            if ii != self.dependencies.len() - 1 {
                write!(result, ", ").unwrap();
            }
        }
        result
    }
}

impl Default for TodoItem {
    fn default() -> Self {
        Self {
            id: 0,
            title: String::new(),
            text: String::new(),
            done: false,
            priority: 0,
            dependencies: BTreeSet::new(),
            parent: None,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<TodoItem>,
    pub id_counter: u32,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            id_counter: 0,
        }
    }

    pub fn read_from<P: AsRef<Path>>(file: P) -> TodoResult<Self> {
        let list = serde_json::from_str(&fs::read_to_string(file)?)?;
        Ok(list)
    }

    pub fn write_to<P: AsRef<Path>>(&self, file: P) -> TodoResult<()> {
        fs::write(file, serde_json::to_vec(self)?)?;
        Ok(())
    }

    pub fn add_new(&mut self, title: String, priority: i16, done: bool) -> &TodoItem {
        let id = self.id_counter;
        self.id_counter += 1;
        self.todos.push(TodoItem {
            id,
            title,
            text: String::new(),
            priority,
            done,
            dependencies: BTreeSet::new(),
            parent: None,
        });
        self.todos.last().unwrap()
    }

    pub fn by_id(&self, id: u32) -> Option<&TodoItem> {
        self.todos.iter().find(|item| item.id == id)
    }

    pub fn by_id_mut(&mut self, id: u32) -> Option<&mut TodoItem> {
        self.todos.iter_mut().find(|item| item.id == id)
    }

    pub fn remove(&mut self, id: u32) -> Option<TodoItem> {
        let index = self.todos.iter().position(|item| item.id == id);
        if let Some(index) = index {
            Some(self.todos.remove(index))
        } else  {
            None
        }
    }
}
