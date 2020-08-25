use super::prelude::*;
use ansi_term::Style;
use clap::value_t;

pub struct Show;
impl Command for Show {
    const NAME: &'static str = "show";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Shows details for the specified task")
            .arg(Arg::with_name("ID").required(true).index(1))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let id: u32 = value_t!(matches.value_of("ID"), u32).unwrap_or_else(|e| e.exit());
        let todo_list = app.try_load_current()?;
        let item = todo_list.by_id(id).ok_or(TodoError::ItemNotFound(id))?;
        println!("Item #{}:", id);
        println!("{}", Style::new().bold().paint(&item.title));
        if !item.dependencies.is_empty() {
            println!("-> depends on {}", item.format_depends(&todo_list, true));
        }
        println!();
        println!("{}", item.text.trim());
        Ok(())
    }
}
