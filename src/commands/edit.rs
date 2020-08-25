use super::prelude::*;

use clap::value_t;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::process::{Command as PCommand, Stdio};

fn get_editor() -> Cow<'static, str> {
    match std::env::var("VISUAL") {
        Ok(editor) => editor.into(),
        Err(_) => "vi".into(),
    }
}

fn compute_editor_args<'a>(editor_name: &str, file_name: &'a str) -> Vec<&'a str> {
    match editor_name {
        "vi" | "vim" | "nvim" => vec![file_name],
        "code" | "code-insiders" | "vscode" => vec!["--wait", file_name],
        _ => {
            eprintln!("Warning: Unsupported editor {}", editor_name);
            vec![file_name]
        }
    }
}

pub fn edit_item(item: &mut TodoItem) -> TodoResult<()> {
    let editor = get_editor();
    let file = std::env::temp_dir().join("todo_editor.txt");

    {
        let mut file = BufWriter::new(File::create(&file)?);
        writeln!(file, "{}", item.title)?;
        writeln!(file)?;
        writeln!(file, "{}", item.text)?;
    }

    let status = PCommand::new(editor.as_ref())
        .args(&compute_editor_args(&editor, &file.to_string_lossy()))
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .stdin(Stdio::null())
        .status()?;
    if !status.success() {
        return Err(TodoError::EditorExitCode(status));
    }

    {
        let file = BufReader::new(File::open(&file)?);
        let mut lines = file.lines();

        item.title = lines.next().ok_or(TodoError::EditorEmptyFile)??;

        let mut text = String::new();

        if let Some(empty_line) = lines.next() {
            let empty_line = empty_line?;
            if !empty_line.trim().is_empty() {
                text.push_str(&empty_line);
                text.push('\n');
            }
        }

        for line in lines {
            text.push_str(&line?);
            text.push('\n');
        }

        item.text = text;
    }

    std::fs::remove_file(file)?;
    Ok(())
}

pub struct Edit;
impl Command for Edit {
    const NAME: &'static str = "edit";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME)
            .about("Edit the specified task's title and description")
            .arg(Arg::with_name("ID").required(true))
    }

    fn invoke(app: &AppContext, matches: &ArgMatches) -> TodoResult<()> {
        let id: u32 = value_t!(matches.value_of("ID"), u32).unwrap_or_else(|e| e.exit());
        let mut todo_list = app.try_load_current()?;
        let item = todo_list.by_id_mut(id).ok_or(TodoError::ItemNotFound(id))?;
        edit_item(item)?;
        app.write_current(&todo_list)?;
        Ok(())
    }
}
