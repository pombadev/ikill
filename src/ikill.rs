use heim::process::{processes, Process};
use skim::prelude::*;
use std::io::Cursor;
use tokio::stream::StreamExt;

pub async fn run() {
    let options = SkimOptionsBuilder::default()
        .height(Some("70%"))
        .reverse(true)
        .multi(true)
        .build()
        .unwrap();

    let processes = processes();

    tokio::pin!(processes);

    let all_processes: Vec<Process> = processes
        .map(|item| item.expect("Unable to unwrap process"))
        .collect()
        .await;

    let mut input = String::new();

    for ps in &all_processes {
        let pid = ps.pid();
        let name = match ps.name().await {
            Ok(name) => name,
            Err(error) => {
                eprint!("{}", error.to_string());
                std::process::exit(1);
            }
        };

        input.push_str(format!("{} {}\n", name, pid).as_str())
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input.to_string()));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let selected_pids = selected_items
        .iter()
        .map(|item| {
            // returns str like: "command_name pid"
            let text = item.text();
            let mut pieces = text.split_whitespace();
            // skip name
            let _name = pieces.next();

            match pieces.next() {
                Some(pid) => pid.to_string(),
                None => "".to_string(),
            }
        })
        .collect::<Vec<String>>();

    for process in all_processes {
        // pid is i32, .to_string() will convert it to String
        let selected_process = selected_pids.contains(&process.pid().to_string());

        if selected_process {
            match process.terminate().await {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("error: {}", error.to_string());
                }
            }
        }
    }
}
