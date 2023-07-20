use std::ops::Div;

use clap::Args;

use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Cell, Color, Row,
};

use crate::state::{state_dir, ProcState, ProcStatus};

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long)]
    /// Hide offline processes
    exclude_offline: bool,
}

impl ListArgs {
    pub async fn run(self) -> anyhow::Result<()> {
        let mut process_dirs = state_dir().join("procs").read_dir().unwrap();
        let mut rows = vec![];

        while let Some(Ok(process)) = process_dirs.next() {
            let state = ProcState::receive(process.file_name().to_str().unwrap()).unwrap();
            let items = match state.status {
                ProcStatus::Running(pid, mut process) => {
                    vec![
                        pid.to_string(),
                        state.name,
                        "online".to_string(),
                        format!("{}%", process.cpu_percent().unwrap()),
                        format!("{}-Mb", process.memory_info().unwrap().vms().div(1_048_476)),
                        format!(
                            "{} m",
                            state.start_time.elapsed().unwrap().as_secs().div(60)
                        ),
                    ]
                }
                ProcStatus::Stopped => {
                    if self.exclude_offline {
                        continue;
                    }
                    vec![
                        "0".to_string(),
                        state.name,
                        "offline".to_string(),
                        "0%".to_string(),
                        "0-Mb".to_string(),
                        "0 m".to_string(),
                    ]
                }
            };
            let cells: Vec<Cell> = items.into_iter().map(Cell::from).collect();

            let row = Row::from(cells);
            rows.push(row);
        }
        let mut table = comfy_table::Table::new();
        table
            .set_header(
                vec!["pid", "name", "status", "cpu", "mem", "uptime"]
                    .iter()
                    .map(|i| Cell::from(i).fg(Color::Red))
                    .collect::<Vec<Cell>>(),
            )
            .add_rows(rows)
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .apply_modifier(UTF8_SOLID_INNER_BORDERS);

        println!("{}", table);

        Ok(())
    }
}
