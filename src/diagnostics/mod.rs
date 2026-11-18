pub mod health;
pub mod storage;
pub mod system;

pub fn run_diagnostics() {
    println!("\n========== FORGEKIT DIAGNOSTICS ==========\n");

    system::show_info();
    system::show_memory();
    storage::show_storage();
    health::show_health();

    println!("\n========== DIAGNOSTICS COMPLETE ==========");
}
