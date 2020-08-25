use super::prelude::*;
use clap::value_t;

pub struct Parent;
impl Command for Parent {
    const NAME: &'static str = "parent";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Sets the specified task's parent")
            .arg(Arg::with_name("ID").required(true).index(1))
            .arg(Arg::with_name("PARENT").required(true).index(2))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let id: u32 = value_t!(matches.value_of("ID"), u32).unwrap_or_else(|e| e.exit());
        let parent: u32 = value_t!(matches.value_of("PARENT"), u32).unwrap_or_else(|e| e.exit());
        let mut todo_list = app.try_load_current()?;
        let item = todo_list.by_id_mut(id).ok_or(TodoError::ItemNotFound(id))?;
        item.parent = Some(parent);
        app.write_current(&todo_list)?;
        Ok(())
    }
}
