use super::prelude::*;
use regex::Regex;

pub struct New;
impl Command for New {
    const NAME: &'static str = "new";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Create a new Todo list")
            .arg(Arg::with_name("NAME").required(true).index(1))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let name = matches.value_of("NAME").unwrap();
        if !Regex::new(r"[a-zA-Z0-9_\-]+").unwrap().is_match(name) {
            return Err(TodoError::InvalidName);
        }
        let path = app.path_for(name)?;
        if path.exists() {
            return Err(TodoError::ListAlreadyExists(name.into()));
        }
        TodoList::new().write_to(&path)?;
        app.set_current_todo_name(name)?;
        println!("Created list '{}'", name);
        Ok(())
    }
}
