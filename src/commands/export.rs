use super::prelude::*;
use std::fmt::Write as _;
use std::fs::{create_dir_all, File};
use std::io::{stdout, Write};
use std::path::PathBuf;

pub struct Export;
impl Command for Export {
    const NAME: &'static str = "export";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Export the current Todo list")
            .arg(Arg::with_name("OUTPUT")
                 .default_value("-")
                 .index(1)
                 .help("The output file (for dot) or directory (for html). Defaults to standard out for dot"))
            .arg(Arg::with_name("format")
                 .required(true)
                 .short("F")
                 .long("format")
                 .takes_value(true)
                 .possible_values(&["html", "dot"])
                 .help("The output format"))
            .arg(Arg::with_name("force").short("f").long("force").help("Overwrite existing files"))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let format = matches.value_of("format").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap().trim();
        if output_file.is_empty() {
            panic!("Empty argument")
        }

        let force = matches.is_present("force");

        let list = app.try_load_current()?;
        let name = app.get_current_todo_name()?.unwrap();

        match format {
            "html" => {
                let output_dir = PathBuf::from(output_file);
                if !output_dir.exists() {
                    create_dir_all(&output_dir)?;
                } else {
                    assert!(output_dir.is_dir(), "html output file must be a directory");
                    if !force && output_dir.read_dir()?.next().is_some() {
                        panic!("Specified output directory is not empty");
                    }
                }

                // Generate index.html: Task list
                {
                    let mut index_html_file = File::create(output_dir.join("index.html"))?;
                    let mut task_list = String::new();
                    for item in list.todos {
                        let mut classes = vec!["list-item"];
                        if item.done {
                            classes.push("done");
                        }
                        writeln!(task_list, "<li class=\"{}\">#{} - {}</li>", classes.join(" "), item.id, item.title).unwrap();
                    }

                    write!(
                        index_html_file,
                        include_str!("index.template.html"),
                        todo_list_name = name,
                        task_list = task_list,
                    )?;
                }

                // Generate index.css
                {
                    let mut index_css_file = File::create(output_dir.join("index.css"))?;
                    write!(
                        index_css_file,
                        "{}",
                        include_str!("index.template.css"),
                    )?;
                }
            }
            "dot" => {
                let mut out: Box<dyn Write> = match output_file {
                    "-" => Box::new(stdout()),
                    file => {
                        let file = PathBuf::from(file);
                        if !force && file.exists() {
                            panic!("File already exists");
                        }
                        Box::new(File::create(file)?)
                    }
                };

                write!(out, "digraph \"{}\" {{", name)?;
                for item in list.todos {
                    write!(out, "\t\"{}\" [label = \"{}\"];", item.id, item.title)?;
                    for dep in item.dependencies {
                        write!(out, "\t\"{}\" -> \"{}\" [style = dotted];", item.id, dep)?;
                    }
                    if let Some(parent) = item.parent {
                        write!(out, "\t\"{}\" -> \"{}\";", item.id, parent)?;
                    }
                }
                write!(out, "}}")?;
            }
            _ => unreachable!(),
        }
        Ok(())
    }
}
