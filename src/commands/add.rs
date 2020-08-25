use super::prelude::*;

pub struct Add;
impl Command for Add {
    const NAME: &'static str = "add";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Adds a Todo item to the current list")
            .arg(Arg::with_name("TEXT").required(true).index(1))
            .arg(
                Arg::with_name("priority")
                    .short("p")
                    .long("priority")
                    .value_name("PRIORITY"),
            )
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let text = matches.value_of("TEXT").unwrap();
        let priority = matches
            .value_of("priority")
            .map(|p| p.parse::<i16>().unwrap())
            .unwrap_or(0);

        let mut todo_list = app.try_load_current()?;
        let todo = todo_list.add_new(text.into(), priority, false);
        println!("Added Todo item #{}", todo.id);
        app.write_current(&todo_list)?;
        Ok(())
    }
}
