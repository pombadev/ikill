use futures_lite::StreamExt;
use heim::process::processes;
use skim::prelude::*;

/*
(date; ps -ef) |
  fzf --bind='ctrl-r:reload(date; ps -ef)' \
      --header=$'Press CTRL-R to reload\n\n' --header-lines=2 \
      --preview='echo {}' --preview-window=down,3,wrap \
      --layout=reverse --height=80% | awk '{print $2}' | xargs kill -9
*/

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
        .inline_info(true)
        .preview_window(Some("right:50%"))
        .bind(vec!["ctrl-l:unix-line-discard"])
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

    if let Some(out) = Skim::run_with(&skim_options, Some(rx)) {
        // exit when `esc` key were pressed
        if out.is_abort {
            return Ok(());
        }

        for item in out.selected_items {
            let pid = item.output();

            let selected = all_processes.iter().find(|ps|
                    // unwraping this because this was converted from i32
                    ps.pid() == pid.parse::<i32>().unwrap());

            if let Some(selected) = selected {
                selected.kill().await?;
            }
        }
    }

    Ok(())
}
