use heim::process::Process;
use skim::prelude::*;
use std::io::Cursor;

pub async fn run(all_processes: Vec<Process>) {
    let options = SkimOptionsBuilder::default()
        .height(Some("70%"))
        .reverse(true)
        .build()
        .unwrap();

    let mut input = String::new();

    for ps in &all_processes {
        let pid = ps.pid();
        let name = ps.name().await.unwrap();

        input.push_str(format!("{} {}\n", name, pid).as_str())
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input.to_string()));

    let selected_item = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let selected: String = selected_item.iter().map(|item| item.text()).collect();

    // if `esc` is pressed no selection will be made, therefore this will be empty.
    if selected.is_empty() {
        return;
    }

    let name_and_pid: Vec<&str> = selected.split(' ').collect();

    let selected_pid = name_and_pid
        .get(1)
        .expect("Unable to get PID")
        .parse::<i32>()
        .unwrap();

    let selected_process = all_processes
        .iter()
        .find(|item| item.pid() == selected_pid)
        .unwrap();

    match selected_process.terminate().await {
        Ok(_) => {}
        Err(error) => {
            eprintln!("error: {}", error.to_string());
        }
    }
}
