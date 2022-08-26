use super::error::{TodoError, TodoResult};
use super::todo::TodoList;
use std::cell::RefCell;
use std::path::PathBuf;
use xdg::BaseDirectories;

pub struct AppContext {
    base_dirs: BaseDirectories,
    current_todo_cache: RefCell<Option<String>>,
}

impl AppContext {
    pub fn create() -> Self {
        Self {
            base_dirs: BaseDirectories::with_prefix("todo").unwrap(),
            current_todo_cache: RefCell::new(None),
        }
    }

    pub fn get_current_todo_name(&self) -> TodoResult<Option<String>> {
        {
            let cached = self.current_todo_cache.borrow();
            if cached.is_some() {
                return Ok(cached.clone());
            }
        }
        let name = self
            .base_dirs
            .find_data_file("current_set")
            .map(std::fs::read_to_string)
            .transpose()?;
        if let Some(ref name) = name {
            self.current_todo_cache.replace(Some(name.clone()));
        }
        Ok(name)
    }

    pub fn path_for<N: AsRef<str>>(&self, name: N) -> TodoResult<PathBuf> {
        Ok(self
            .base_dirs
            .place_data_file(&format!("{}.json", name.as_ref()))?)
    }

    pub fn current_path(&self) -> TodoResult<Option<PathBuf>> {
        self
            .get_current_todo_name()?
            .map(|name| self.path_for(name))
            .transpose()
    }

    pub fn set_current_todo_name(&self, name: &str) -> TodoResult<()> {
        std::fs::write(self.base_dirs.place_data_file("current_set")?, name)?;
        self.current_todo_cache.replace(Some(name.to_owned()));
        Ok(())
    }

    pub fn try_load_current(&self) -> TodoResult<TodoList> {
        let current_path = self.current_path()?.ok_or(TodoError::NoActiveList)?;
        TodoList::read_from(current_path)
    }

    pub fn write_current(&self, list: &TodoList) -> TodoResult<()> {
        list.write_to(self.current_path()?.unwrap())?;
        Ok(())
    }
}
