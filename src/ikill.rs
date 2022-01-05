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

    let input = all_processes.iter().fold(String::new(), |mut acc, ps| {
        smol::block_on(async {
            if let Ok(name) = ps.name().await {
                acc.push_str(format!("{} {}\n", name, ps.pid()).as_str());
            }
        });

        acc
    });

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        // skip items where `esc` key were pressed
        .filter(|out| !out.is_abort)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    let mut selected_pids = selected_items
        .iter()
        .map(|item| {
            item.text()
                .split_whitespace()
                .skip(1)
                .fold(String::with_capacity(0), |_, curr| curr.into())
        });

    for process in all_processes {
        let selected_process = selected_pids.any(|x| x == process.pid().to_string());

        if selected_process {
            if let Err(error) = process.terminate().await {
                eprintln!("Error: {}", error.to_string());
            }
        }
    }
}
