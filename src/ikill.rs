use heim::process::processes;
use skim::prelude::*;
use smol::stream::StreamExt;
use std::io::Cursor;

pub async fn run() {
    let options = SkimOptionsBuilder::default()
        .height(Some("70%"))
        .reverse(true)
        .multi(true)
        .build()
        .unwrap();

    let all_processes = match processes().await {
        Ok(processes) => processes.filter_map(|process| process.ok()).collect().await,
        Err(_) => Vec::with_capacity(0),
    };

    let mut input = String::new();

    for ps in &all_processes {
        if let Ok(name) = ps.name().await {
            input.push_str(format!("{} {}\n", name, ps.pid()).as_str());
        }
    }

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        // skip items where `esc` key were pressed
        .filter(|out| !out.is_abort)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let selected_pids = selected_items
        .iter()
        .map(|item| {
            item.text()
                .split_whitespace()
                .skip(1)
                .fold(String::with_capacity(0), |_, curr| curr.into())
        })
        .collect::<Vec<String>>();

    for process in all_processes {
        // pid is i32, .to_string() will convert it to String
        let selected_process = selected_pids.contains(&process.pid().to_string());

        if selected_process {
            match process.terminate().await {
                Ok(_) => {}
                Err(error) => {
                    eprintln!("Error: {}", error.to_string());
                }
            }
        }
    }
}
