use sysinfo::System;

fn main() {
    let mut system = System::new();
    system.refresh_memory();
    
    println!("\n\t\t\tRAM Usage");
    println!("\t+-----------------------------------+");
    println!(
        "\t| Used: \x1B[92m{:.2} GB\x1B[0m / Total: \x1B[93m{:.2} GB\x1B[0m |",
        system.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0,
        system.total_memory() as f64 / 1024.0 / 1024.0 / 1022.0
    );
    println!("\t+-----------------------------------+\n");
}