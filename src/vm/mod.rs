use serde_json::Value;
use std::io::Read;
use virt::connect::Connect;
use virt::domain::Domain;

use crate::vm::qmp::start_vm;

//use crate::helpers;

pub mod qmp;
pub mod types;
pub mod utils;

pub fn create_vm(
    name: &str,
    username: &str,
    password: &str,
    memory: u64,
    vcpus: u8,
    disk_size: u64,
) {
    let seed_iso_path = utils::create_seed_iso(name, username, password);
    let disk_path = utils::create_qemu_img_disk(name, disk_size);

    // current working directory + ./vm-data/{name}.sock
    let qmp_socket = std::env::current_dir()
        .unwrap()
        .join("vm-data")
        .join(format!("{}.sock", name))
        .to_string_lossy()
        .to_string();

    start_vm(&disk_path, &seed_iso_path, &qmp_socket);
}

pub fn boot_vm(name: &str) {
    println!("Booting VM: {}", name);
}

pub fn delete_vm(name: &str) {
    println!("Deleting VM: {}", name);
}

pub fn list_vms() {
    println!("Listing all VMs");
}

pub fn shutdown_vm(name: &str) {
    println!("Shutting down VM: {}", name);
    let qmp_path = std::env::current_dir()
        .unwrap()
        .join("vm-data")
        .join(format!("{}.sock", name))
        .to_string_lossy()
        .to_string();
    let mut stream = qmp::connect_qmp(&qmp_path);

    qmp::init_qmp(&mut stream);

    std::thread::sleep(std::time::Duration::from_millis(100));

    qmp::kill_vm(&mut stream);
    println!("Sent kill signal to VM via QMP");
}

pub fn restart_vm(name: &str) {
    println!("Restarting VM: {}", name);
}

pub fn vm_info(name: &str) {
    println!("Getting info for VM: {}", name);

    let qmp_path = std::env::current_dir()
        .unwrap()
        .join("vm-data")
        .join(format!("{}.sock", name))
        .to_string_lossy()
        .to_string();

    let mut stream = qmp::connect_qmp(&qmp_path);

    // 2. Init QMP
    qmp::init_qmp(&mut stream);

    let mut buffer = [0; 4096];
    let _ = stream.read(&mut buffer);

    // 3. Query status
    let response = qmp::query_vm_status(&mut stream).expect("Failed to query VM status via QMP");

    // 4. Parse JSON
    let v: Value = serde_json::from_str(&response).expect("Failed to parse QMP response as JSON");

    let status = v["return"]["status"].as_str().unwrap_or("unknown");
    let running = v["return"]["running"].as_bool().unwrap_or(false);
    let singlestep = v["return"]["singlestep"].as_bool().unwrap_or(false);

    // 5. Print useful info
    println!("================ VM INFO ================");
    println!("State       : {}", status);
    println!("Running     : {}", running);
    println!("Single Step : {}", singlestep);

    // Optional: derive a simpler interpretation
    let interpreted = match status {
        "running" => "VM is actively executing",
        "paused" => "VM is paused",
        "shutdown" => "VM is powered off",
        "internal-error" => "VM crashed or failed",
        _ => "Unknown state",
    };

    println!("Meaning     : {}", interpreted);
    println!("=========================================");
}
