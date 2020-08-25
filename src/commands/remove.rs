use super::prelude::*;
use clap::value_t;
use std::io::{self, stdin, stdout, Write, BufRead};

pub struct Remove;
impl Command for Remove {
    const NAME: &'static str = "remove";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Removes a Todo item")
            .arg(Arg::with_name("ID").required(true).index(1))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let id: u32 = value_t!(matches.value_of("ID"), u32).unwrap_or_else(|e| e.exit());
        let mut todo_list = app.try_load_current()?;
        if let Some(item) = todo_list.by_id(id) {
            print!("Really delete item #{} ({})? [y/N]: ", item.id, item.title);
            stdout().flush()?;
            if read_confirmation()? {
                todo_list.remove(id).unwrap();
                app.write_current(&todo_list)?;
            }
            Ok(())
        } else {
            Err(TodoError::ItemNotFound(id))
        }
    }
}

fn read_confirmation() -> io::Result<bool> {
    let stdin = stdin();
    let mut stdin = stdin.lock();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let line = line.trim().to_lowercase();
        match line.as_str() {
            "y" | "yes" | "true" => break Ok(true),
            "" | "n" | "no" | "false" => break Ok(false),
            l => {
                print!("Invalid answer '{}', enter [y/N]: ", l);
                stdout().flush()?;
                continue
            }
        }
    }
}
