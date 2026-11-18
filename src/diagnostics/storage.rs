use sysinfo::Disks;

pub fn show_storage() {
    let disks = Disks::new_with_refreshed_list();

    println!("\n--- STORAGE ---");

    if disks.is_empty() {
        println!("No storage devices detected.");
        return;
    }

    for disk in disks.list() {
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total.saturating_sub(available);

        println!("\nDevice: {}", disk.name().to_string_lossy());
        println!("Mount point: {}", disk.mount_point().display());
        println!("File system: {}", disk.file_system().to_string_lossy());
        println!("Total: {:.2} GB", to_gb(total));
        println!("Used: {:.2} GB", to_gb(used));
        println!("Free: {:.2} GB", to_gb(available));

        if total > 0 {
            println!("Usage: {:.1}%", used as f64 / total as f64 * 100.0);
        }
    }
}

fn to_gb(bytes: u64) -> f64 {
    bytes as f64 / 1_000_000_000.0
}
