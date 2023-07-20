
use clap::Args;

use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Cell, Color, Row,
};

use crate::state::{state_dir, ProcessState};

#[derive(Args)]
pub struct ListArgs;

impl ListArgs {
    pub async fn run(self) -> anyhow::Result<()> {
        let mut process_dirs = state_dir().join("processes").read_dir()?;
        let mut rows = vec![];

        while let Some(Ok(process)) = process_dirs.next() {
            let (state, _handle) =
                ProcessState::refresh_process_state(process.file_name().to_str().unwrap());

            let pid = state
                .pid
                .and_then(|pid| Some(pid.to_string()))
                .unwrap_or("unset".to_string());
            let status = match state.pid {
                Some(_) => "online",
                None => "offline",
            };

            let items = vec![
                pid,
                state.name,
                status.to_string(),
                format!("{}%", state.cpu),
                state.mem.to_string(),
                ((state.uptime.as_secs() / 60) as u64).to_string(),
            ];
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
