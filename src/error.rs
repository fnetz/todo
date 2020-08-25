use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::io;
use std::process::ExitStatus;

#[derive(Debug)]
pub enum TodoError {
    NoActiveList,
    ItemNotFound(u32),
    InvalidName,
    ListNotFound(String),
    ListAlreadyExists(String),
    EditorExitCode(ExitStatus),
    EditorEmptyFile,
    Io(io::Error),
    Serde(serde_json::Error),
}

impl Display for TodoError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoActiveList => f.write_str("No list is active. Create one with `new`"),
            Self::ItemNotFound(id) => write!(f, "Todo item not found: #{}", id),
            Self::InvalidName => f.write_str("Invalid name given"),
            Self::ListNotFound(list) => write!(f, "List '{}' does not exist", list),
            Self::ListAlreadyExists(list) => write!(f, "List '{}' already exists", list),
            Self::EditorExitCode(status) => {
                write!(f, "Editor didn't exit successfully: {}", status)
            }
            Self::EditorEmptyFile => f.write_str("File was empty. Aborting ..."),
            Self::Io(err) => write!(f, "IO error: {}", err),
            Self::Serde(err) => write!(f, "Serialization error: {}", err),
        }
    }
}

impl Error for TodoError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::Serde(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for TodoError {
    fn from(err: io::Error) -> TodoError {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for TodoError {
    fn from(err: serde_json::Error) -> TodoError {
        Self::Serde(err)
    }
}

pub type TodoResult<T> = Result<T, TodoError>;
