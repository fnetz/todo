use super::prelude::*;

pub struct Lists;
impl Command for Lists {
    const NAME: &'static str = "lists";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Lists all Todo lists")
    }

    fn invoke(app: &AppContext, _matches: &ArgMatches) -> TodoResult<()> {
        let path = app.path_for("tmp")?;
        let path = path.parent().unwrap();
        for file in path.read_dir()? {
            let file = file?;
            let name = file.file_name();
            let name = name.to_string_lossy();
            if file.metadata()?.is_file() && name != "current_set" {
                let name = name.trim_end_matches(".json");
                println!("{}", name);
            }
        }
        Ok(())
    }
}
