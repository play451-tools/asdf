use sysinfo::{System, RefreshKind, CpuRefreshKind};
use prettytable::{Table, format::consts::FORMAT_NO_LINESEP, row};
use crossterm::{
    event::EnableMouseCapture,
    execute,
    queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType},
};
use std::io::stdout;

fn main() {
    let mut system = System::new_with_specifics(
        RefreshKind::nothing().with_cpu(CpuRefreshKind::everything()),
    );
    system.refresh_memory();
    system.refresh_cpu_all();
    
    let cpus: Vec<_> = system.cpus().into_iter().collect();
    if let Some(first_cpu) = cpus.first() {
        let memory_used = system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        let memory_total = system.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
        
        let mut table = Table::new();
        table.set_format(*FORMAT_NO_LINESEP);
        table.set_titles(row![bF=> "RAM", "CPU"]);
        
        table.add_row(row![bF=> 
            format!("Used: {:.2} GB / Total: {:.2} GB", memory_used, memory_total),
            format!("Brand: {} ({} cores)", 
                first_cpu.brand(), 
                cpus.len(), 
                )
        ]);
        
        let mut stdout = stdout();
        queue!(
            stdout,
            Clear(ClearType::All),
            SetForegroundColor(Color::Green),
            Print(format!("{}", table)),
            ResetColor,
        ).unwrap();
    }
}