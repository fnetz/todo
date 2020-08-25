use clap::{App, AppSettings, ArgMatches};

mod todo;

mod app;
use app::AppContext;

mod commands;
use commands as cmd;
use commands::Command;

mod error;
use error::TodoResult;

fn invoke_command(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
    let (name, matches) = matches.subcommand();
    let matches = matches.unwrap();
    let invoke = match name {
        cmd::New::NAME => cmd::New::invoke,
        cmd::Export::NAME => cmd::Export::invoke,
        cmd::Lists::NAME => cmd::Lists::invoke,
        cmd::Add::NAME => cmd::Add::invoke,
        cmd::List::NAME => cmd::List::invoke,
        cmd::Show::NAME => cmd::Show::invoke,
        cmd::Done::NAME => cmd::Done::invoke,
        cmd::Undone::NAME => cmd::Undone::invoke,
        cmd::Edit::NAME => cmd::Edit::invoke,
        cmd::Use::NAME => cmd::Use::invoke,
        cmd::Depends::NAME => cmd::Depends::invoke,
        cmd::Priority::NAME => cmd::Priority::invoke,
        cmd::Parent::NAME => cmd::Parent::invoke,
        cmd::Unparent::NAME => cmd::Unparent::invoke,
        cmd::Remove::NAME => cmd::Remove::invoke,
        _ => unreachable!("Internal error: Command {} should be handled", name),
    };
    invoke(app, &matches)
}

#[cfg(windows)]
fn enable_ansi() -> Result<(), u32> {
    ansi_term::enable_ansi_support();
}

#[cfg(not(windows))]
fn enable_ansi() -> Result<(), u32> {
    Ok(())
}

fn main() {
    let commands = vec![
        cmd::New::subcommand(),
        cmd::Lists::subcommand(),
        cmd::Export::subcommand(),
        cmd::Add::subcommand(),
        cmd::List::subcommand(),
        cmd::Show::subcommand(),
        cmd::Done::subcommand(),
        cmd::Undone::subcommand(),
        cmd::Edit::subcommand(),
        cmd::Use::subcommand(),
        cmd::Depends::subcommand(),
        cmd::Priority::subcommand(),
        cmd::Parent::subcommand(),
        cmd::Unparent::subcommand(),
        cmd::Remove::subcommand(),
    ];
    let matches = App::new("todo")
        .setting(AppSettings::SubcommandRequired)
        .subcommands(commands)
        .get_matches();

    let app = AppContext::create();

    enable_ansi().unwrap();

    let result = invoke_command(&app, &matches);
    if let Err(err) = result {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}
