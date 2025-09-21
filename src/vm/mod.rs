use virt::connect::Connect;
use virt::domain::Domain;

//use crate::helpers;

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

    let domain_xml =
        utils::generate_installation_domain_xml(name, memory, vcpus, disk_path, seed_iso_path);

    let mut conn = Connect::open(Some("qemu:///system")).unwrap();

    let domain = Domain::define_xml(&mut conn, &domain_xml).unwrap();

    domain.create().unwrap();
}

pub fn boot_vm(name: &str) {
    println!("Booting VM: {}", name);

    let mut conn = Connect::open(Some("qemu:///system")).unwrap();
    let domain = Domain::lookup_by_name(&mut conn, name).unwrap();
    domain.create().unwrap();
}

pub fn delete_vm(name: &str) {
    println!("Deleting VM: {}", name);
    let mut conn = Connect::open(Some("qemu:///system")).unwrap();
    let domain = Domain::lookup_by_name(&mut conn, name).unwrap();
    if domain.is_active().unwrap() {
        domain.destroy().unwrap();
    }

    domain.undefine().unwrap();
}

pub fn list_vms() {
    println!("Listing all VMs");
    let conn = Connect::open(Some("qemu:///system")).unwrap();
    let domains = conn.list_all_domains(0).unwrap();
    for domain in domains {
        let name = domain.get_name().unwrap();
        let id = domain.get_id().unwrap_or(0); // 0 means inactive
        let is_active = domain.is_active().unwrap();
        println!(
            "Name: {}, ID: {}, Active: {}",
            name,
            if id == 0 {
                "N/A".to_string()
            } else {
                id.to_string()
            },
            is_active
        );
    }
}

pub fn shutdown_vm(name: &str) {
    println!("Shutting down VM: {}", name);
    let mut conn = Connect::open(Some("qemu:///system")).unwrap();
    let domain = Domain::lookup_by_name(&mut conn, name).unwrap();

    if domain.is_active().unwrap() {
        domain.shutdown().unwrap();
        let mut timeout = 10; // seconds
        while domain.is_active().unwrap() && timeout > 0 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            timeout -= 1;
        }
        if domain.is_active().unwrap() {
            println!("Graceful shutdown timed out, forcing power off.");
            domain.destroy().unwrap();
        } else {
            println!("Domain {} has been shut down gracefully.", name);
        }
    } else {
        println!("Domain {} is not active.", name);
    }
}

pub fn restart_vm(name: &str) {
    println!("Restarting VM: {}", name);
    let mut conn = Connect::open(Some("qemu:///system")).unwrap();
    let domain = Domain::lookup_by_name(&mut conn, name).unwrap();
    domain.reboot(0).unwrap();
}

pub fn vm_info(name: &str) {
    println!("Getting info for VM: {}", name);

    let conn = Connect::open(Some("qemu:///system")).unwrap();
    let domain = Domain::lookup_by_name(&conn, name).unwrap();

    // state (the tuple contents/shape depend on the binding; printing for debugging)
    if let Ok(state) = domain.get_state() {
        println!("State code: {:?}, reason: {:?}", state.0, state.1);
    }

    // mem / vcpus
    if let Ok(max_mem) = domain.get_max_memory() {
        println!("Max memory: {} KiB", max_mem);
    }
    if let Ok(vcpus) = domain.get_max_vcpus() {
        println!("vCPUs: {}", vcpus);
    }
}
