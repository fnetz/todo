use super::prelude::*;
use regex::Regex;

pub struct Use;
impl Command for Use {
    const NAME: &'static str = "use";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Marks the specified Todo List as active")
            .arg(Arg::with_name("NAME").required(true).index(1))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let name = matches.value_of("NAME").unwrap();
        if !Regex::new(r"[a-zA-Z0-9_\-]+").unwrap().is_match(name) {
            return Err(TodoError::InvalidName);
        }
        let path = app.path_for(name)?;
        if !path.exists() {
            return Err(TodoError::ListNotFound(name.into()));
        }
        app.set_current_todo_name(name)?;
        Ok(())
    }
}
