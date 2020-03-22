use heim::process::{processes, Process};
use tokio::stream::StreamExt as _;

mod fkill;

#[tokio::main]
async fn main() {
    let processes = processes();

    tokio::pin!(processes);

    let all_processes: Vec<Process> = processes
        .map(|item| item.expect("Unable to unwrap process"))
        .collect()
        .await;

    fkill::run(all_processes).await;
}
