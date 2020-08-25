use super::prelude::*;
use clap::value_t;

pub struct Unparent;
impl Command for Unparent {
    const NAME: &'static str = "unparent";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Removes the specified task's parent")
            .arg(Arg::with_name("ID").required(true).index(1))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let id: u32 = value_t!(matches.value_of("ID"), u32).unwrap_or_else(|e| e.exit());
        let mut todo_list = app.try_load_current()?;
        let item = todo_list.by_id_mut(id).ok_or(TodoError::ItemNotFound(id))?;
        item.parent = None;
        app.write_current(&todo_list)?;
        Ok(())
    }
}
