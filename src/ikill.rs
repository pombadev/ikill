use heim::process::Process;
use skim::prelude::*;
use std::io::Cursor;

pub async fn run(all_processes: Vec<Process>) {
    let options = SkimOptionsBuilder::default()
        .height(Some("70%"))
        .reverse(true)
        .multi(true)
        .build()
        .unwrap();

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
            let mut pieces = text.split_ascii_whitespace();
            // skip name
            pieces.next();
            // return pid <i32> which will be the pid or -1 so that we can filter it out later
            match pieces.next() {
                None => -1,
                Some(pid) => match pid.parse::<i32>() {
                    Ok(n) => n,
                    Err(_) => -1,
                },
            }
        })
        .collect::<Vec<i32>>();

    for process in all_processes {
        let selected_process = selected_pids.contains(&process.pid());

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
