use clap::Args;
use comfy_table::{
    modifiers::{UTF8_ROUND_CORNERS, UTF8_SOLID_INNER_BORDERS},
    presets::UTF8_FULL,
    Cell, Color, Row,
};
use psutil::{host, process::Process};

use crate::{
    process::{state_dir, Proc},
    PROC_DIR,
};

#[derive(Args)]
pub struct ListArgs {
    #[arg(short, long)]
    /// Hide offline processes
    exclude_offline: bool,
}

impl ListArgs {
    pub fn run(self) -> anyhow::Result<()> {
        let mut process_dirs = state_dir().join(PROC_DIR).read_dir()?;
        let mut rows = vec![];
        while let Some(Ok(process)) = process_dirs.next() {
            let os_name = process.file_name();
            let name = os_name.into_string().unwrap();
            let status = Proc::get(&name)?;
            let items = match status {
                Proc::Running(pid) => {
                    let mut process = Process::new(pid)?;
                    vec![
                        pid.to_string(),
                        name,
                        "online".to_string(),
                        format!("{}%", process.cpu_percent()?),
                        format!("{}-Mb", process.memory_info()?.vms() / 1_048_476),
                        format!(
                            "{} m",
                            (host::uptime()? - process.create_time()).as_secs() / 60
                        ),
                    ]
                }
                Proc::Stopped => {
                    if self.exclude_offline {
                        continue;
                    }
                    vec![
                        "0".to_string(),
                        name,
                        "offline".to_string(),
                        "0%".to_string(),
                        "0-Mb".to_string(),
                        "0 m".to_string(),
                    ]
                }
                Proc::NotFound => continue,
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
