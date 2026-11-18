use sysinfo::{Disks, System};

pub fn show_health() {
    let mut system = System::new();
    system.refresh_memory();

    println!("\n--- BASIC HEALTH CHECK ---");

    let total = system.total_memory();
    let available = system.available_memory();

    if total > 0 {
        let usage = total.saturating_sub(available) as f64 / total as f64 * 100.0;

        if usage >= 90.0 {
            println!("WARNING: High memory usage ({usage:.1}%).");
        } else {
            println!("OK: Memory usage is {usage:.1}%.");
        }
    } else {
        println!("INFO: Memory usage could not be determined.");
    }

    let disks = Disks::new_with_refreshed_list();

    if disks.is_empty() {
        println!("INFO: No storage devices were reported.");
    }

    for disk in disks.list() {
        let total = disk.total_space();
        if total == 0 {
            continue;
        }

        let used = total.saturating_sub(disk.available_space());
        let usage = used as f64 / total as f64 * 100.0;

        if usage >= 90.0 {
            println!(
                "WARNING: Storage usage is {usage:.1}% on {}.",
                disk.mount_point().display()
            );
        } else {
            println!(
                "OK: Storage usage is {usage:.1}% on {}.",
                disk.mount_point().display()
            );
        }
    }

    println!("Health check completed. No changes were made to your system.");
}
