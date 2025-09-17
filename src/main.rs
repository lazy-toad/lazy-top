use sysinfo::{Cpu, Process, System};

// Improvement: Define a constant for clarity instead of using a magic number.
const GIB_DIVISOR: f64 = (1024 * 1024 * 1024) as f64;

fn main() {
    // Use new_all() to initialize and load data at once.
    let mut sys = System::new_all();

    println!("System info refreshing...");
    //Refresh, sleep, then refresh again for accurate CPU readings.
    sys.refresh_all();
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_all();
    println!("System refreshed");

    println!("\n== Global Sys Info ==");
    println!(
        "Total Memory: {:.2} GiB",
        sys.total_memory() as f64 / GIB_DIVISOR
    );
    println!(
        "Used Memory: {:.2} GiB",
        sys.used_memory() as f64 / GIB_DIVISOR
    );
    println!(
        "Total Swap: {:.2} GiB",
        sys.total_swap() as f64 / GIB_DIVISOR
    );
    println!("Used Swap: {:.2} GiB", sys.used_swap() as f64 / GIB_DIVISOR);

    println!("\n\n== CPU Info==");
    println!("Global CPU Usage: {:.2}%", sys.global_cpu_usage());
    println!("Number of cores: {}", sys.cpus().len());

    println!("\n\n== Processes ==");
    let mut processes: Vec<_> = sys.processes().values().collect();
    processes.sort_by(|a, b| {
        b.cpu_usage()
            .partial_cmp(&a.cpu_usage())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    println!("{:<9} | {:<25} | {:<8}", "PID", "NAME", "CPU");
    println!("{:-<9} | {:-<25} | {:-<8}", "", "", "");

    for process in processes.iter().take(10) {
        println!(
            "{:<9} | {:<25.25} | {:>5.2}%",
            process.pid(),
            process.name().to_string_lossy(),
            process.cpu_usage()
        )
    }
}
