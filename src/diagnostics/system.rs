use sysinfo::System;

pub fn show_info() {
    let mut system = System::new_all();
    system.refresh_all();

    println!("\n--- SYSTEM INFORMATION ---");
    println!("OS: {}", System::name().unwrap_or_else(|| "Unknown".into()));
    println!(
        "Hostname: {}",
        System::host_name().unwrap_or_else(|| "Unknown".into())
    );
    println!(
        "OS version: {}",
        System::os_version().unwrap_or_else(|| "Unknown".into())
    );
    println!(
        "Kernel: {}",
        System::kernel_version().unwrap_or_else(|| "Unknown".into())
    );
    println!("Logical CPU cores: {}", system.cpus().len());
}

pub fn show_memory() {
    let mut system = System::new();
    system.refresh_memory();

    let total = system.total_memory();
    let available = system.available_memory();
    let used = total.saturating_sub(available);

    println!("\n--- MEMORY ---");
    println!("Total: {:.2} GiB", to_gib(total));
    println!("Used: {:.2} GiB", to_gib(used));
    println!("Available: {:.2} GiB", to_gib(available));

    if total > 0 {
        println!("Usage: {:.1}%", used as f64 / total as f64 * 100.0);
    } else {
        println!("Usage: unavailable");
    }
}

fn to_gib(bytes: u64) -> f64 {
    bytes as f64 / 1_073_741_824.0
}
