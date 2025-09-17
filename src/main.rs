use sysinfo::{Cpu, Process, System};

fn main() {
    let mut sys = System::new();

    println!("System info refreshing...");
    sys.refresh_all();
    println!("System refreshed");

    println!("\n== Global Sys Info ==");
    println!(
        "Total Memory: {:.2}",
        sys.total_memory() as f64 / 1_073_741_824.0
    );
    println!(
        "Used Memory: {:.2}",
        sys.used_memory() as f64 / 1_073_741_824.0
    );
    println!(
        "Total Swap: {:.2}",
        sys.total_swap() as f64 / 1_073_741_824.0
    );
    println!("Used Swap: {:.2}", sys.used_swap() as f64 / 1_073_741_824.0);

    println!("\n\n== CPU Info==");
    println!("Global CPU Usage: {:.2}", sys.global_cpu_usage());
    println!("Numbe of cores: {}", sys.cpus().len());

    println!("\n\n== Processes ==");
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| {
        b.cpu_usage()
            .partial_cmp(&a.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    for process in processes.iter().take(10) {
        let pid_col = format!("PID: {}", process.pid());
        println!(
            "{:<15} | NAME: {:<25} | CPU: {:>5.2}%",
            pid_col,
            process.name().to_string_lossy(),
            process.cpu_usage()
        );
    }
}
