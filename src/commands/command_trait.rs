use crate::app::AppContext;
use crate::error::TodoResult;
use clap::{App, ArgMatches};

pub trait Command {
    const NAME: &'static str;

    fn subcommand<'a, 'b>() -> App<'a, 'b>;
    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()>;
}
