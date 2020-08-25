use super::prelude::*;
use clap::{value_t, values_t};

pub struct Depends;
impl Command for Depends {
    const NAME: &'static str = "depends";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Add dependencies to a task")
            .arg(Arg::with_name("ID").required(true).index(1))
            .arg(Arg::with_name("DEPENDENCY").multiple(true).required(true))
            .arg(Arg::with_name("remove").short("r").long("remove"))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let id: u32 = value_t!(matches.value_of("ID"), u32).unwrap_or_else(|e| e.exit());
        let deps = values_t!(matches.values_of("DEPENDENCY"), u32).unwrap_or_else(|e| e.exit());

        let mut todo_list = app.try_load_current()?;

        for dep in &deps {
            let dep = *dep;
            if todo_list.by_id(dep).is_none() {
                return Err(TodoError::ItemNotFound(dep));
            }
        }

        let item = todo_list.by_id_mut(id).ok_or(TodoError::ItemNotFound(id))?;

        if matches.is_present("remove") {
            println!("Removing dependencies from {}: {:?}", id, deps);
            for dep in deps {
                if !item.dependencies.remove(&dep) {
                    eprintln!("Warning: #{} was not a dependency of #{}", dep, id);
                }
            }
        } else {
            for dep in deps {
                item.dependencies.insert(dep);
            }
        }

        app.write_current(&todo_list)?;
        Ok(())
    }
}
