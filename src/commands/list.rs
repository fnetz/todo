use super::prelude::*;
use ansi_term::Style;

struct ParentItem<'a> {
    item: &'a TodoItem,
    children: Vec<&'a TodoItem>,
}

fn render_item(item: &TodoItem, list: &TodoList, indent: bool) {
    let mut style = Style::default();
    if item.done {
        style = style.strikethrough();
    } else if item.priority > 0 {
        style = style.bold();
    } else if item.priority < 0 {
        style = style.dimmed();
    }
    let task_char = if item.done { '✔' } else { ' ' };
    if indent {
        print!(
            "     {} #{:<3} {}",
            task_char,
            item.id,
            style.paint(&item.title)
        );
    } else {
        print!("#{:<3} {} {}", item.id, task_char, style.paint(&item.title));
    }
    if !item.done && item.priority != 0 {
        print!(" ({})", item.priority);
    }
    println!();
    if !item.done && !item.dependencies.is_empty() {
        if indent {
            print!("   ");
        }
        println!("       ` depends on: {}", item.format_depends(&list, true));
    }
}

pub struct List;
impl Command for List {
    const NAME: &'static str = "list";

    fn subcommand<'a, 'b>() -> App<'a, 'b> {
        SubCommand::with_name(Self::NAME).about("List the current Todo list's items")
    }

    fn invoke(app: &AppContext, _matches: &ArgMatches) -> TodoResult<()> {
        let todo_list = app.try_load_current()?;
        let name = app.get_current_todo_name()?.unwrap();

        println!("Items of {}:", name);
        if todo_list.todos.is_empty() {
            println!(
                "{}",
                Style::new().dimmed().paint("-- No todo items found --")
            );
        } else {
            let mut render_items = Vec::new();
            for item in todo_list.todos.iter() {
                if item.parent.is_none() {
                    render_items.push(ParentItem {
                        item,
                        children: Vec::new(),
                    });
                }
            }

            for item in todo_list.todos.iter() {
                if let Some(item_id) = item.parent {
                    let parent: &mut ParentItem = render_items
                        .iter_mut()
                        .find(|s| s.item.id == item_id)
                        .unwrap();
                    parent.children.push(item);
                }
            }

            for ParentItem { item, children } in render_items {
                render_item(item, &todo_list, false);
                for child in children {
                    render_item(child, &todo_list, true);
                }
            }
        }
        Ok(())
    }
}
