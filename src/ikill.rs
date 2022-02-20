use heim::process::processes;
use skim::prelude::*;
use smol::stream::StreamExt;

struct SelectedProcess {
    name: String,
    pid: i32,
    cmd: String,
}

impl SkimItem for SelectedProcess {
    fn text(&self) -> Cow<str> {
        let inner = format!("{} {} {}", self.pid, self.name, self.cmd);

        Cow::Owned(inner)
    }

    fn output(&self) -> Cow<str> {
        Cow::Owned(self.pid.to_string())
    }
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let skim_options = SkimOptionsBuilder::default()
        .height(Some("70%"))
        .reverse(true)
        .multi(true)
        .case(CaseMatching::Smart)
        .header(Some("PID NAME COMMAND"))
        .build()?;

    let all_processes = processes()
        .await?
        .filter_map(|process| process.ok())
        .collect::<Vec<_>>()
        .await;

    let (tx, rx): (SkimItemSender, SkimItemReceiver) = unbounded();

    for ps in &all_processes {
        let name = ps.name().await?;
        let pid = ps.pid();
        let cmd = ps.command().await?;
        let cmd = cmd.to_os_string();
        let cmd = cmd.to_string_lossy();

        tx.send(Arc::new(SelectedProcess {
            name,
            pid,
            cmd: cmd.into_owned(),
        }))?;
    }

    drop(tx);

    let selected_items = Skim::run_with(&skim_options, Some(rx))
        // skip items where `esc` key were pressed
        .filter(|out| !out.is_abort)
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    #[allow(clippy::needless_collect)]
    let selected_items = selected_items
        .iter()
        .map(|item| item.output().to_string())
        .collect::<Vec<_>>();

    for process in all_processes {
        let selected_process = selected_items.contains(&process.pid().to_string());

        if selected_process {
            process.kill().await?;
        }
    }

    Ok(())
}
