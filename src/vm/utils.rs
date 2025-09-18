use crate::helpers;
use crate::vm::types;
//use serde_yml;
use sha_crypt::{Sha512Params, sha512_simple};
use std::path::Path;
use std::process::Command;
use uuid::Uuid;

pub fn create_qemu_img_disk(name: &str, size_gb: u64) -> String {
    let image_dir = "/var/lib/libvirt/images";
    let disk_path = format!("{}/{}.qcow2", image_dir, name);
    let cloud_img = format!("{}/iso/noble-server-cloudimg-amd64.img", image_dir);
    let disk_path_obj = Path::new(&disk_path);

    // 2. Ensure the directory exists. This is necessary because the program is creating a file
    // in a system directory. This requires `sudo`.
    if let Some(parent) = disk_path_obj.parent() {
        if !parent.exists() {
            println!("Creating directory: {:?}", parent);
            Command::new("mkdir")
                .arg("-p")
                .arg(parent)
                .status()
                .expect("Failed to create parent directory.");
        }
    }

    let output = Command::new("qemu-img")
        .args(&[
            "create",
            "-f",
            "qcow2",
            "-F",
            "qcow2",
            "-b",
            &cloud_img,
            &disk_path,
            &format!("{}G", size_gb),
        ])
        .output()
        .expect("Failed to execute qemu-img command");

    if output.status.success() {
        println!("Disk image created successfully.");
    } else {
        eprintln!(
            "Error creating disk image: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    println!("Changing ownership of {} to libvirt-qemu...", &disk_path);
    Command::new("chown")
        .arg("libvirt-qemu:libvirt-qemu")
        .arg(&disk_path)
        .status()
        .expect("Failed to execute `chown` command. Please ensure you are running the program with `sudo`.");

    format!("{}", disk_path)
}

pub fn generate_installation_domain_xml(
    name: &str,
    memory: u64,
    vcpus: u8,
    disk_path: String,
    seed_iso_path: String,
) -> String {
    let domain_uuid = Uuid::new_v4().to_string();

    let memory_config = types::Memory {
        unit: "MiB".to_string(),
        value: memory.to_string(),
    };
    let current_memory_config = types::Memory {
        unit: "MiB".to_string(),
        value: memory.to_string(),
    };
    let vcpu_config = types::Vcpu {
        placement: "static".to_string(),
        value: vcpus.to_string(),
    };

    let iso_disk = types::Disk {
        device: "cdrom".to_string(),
        disk_type: "file".to_string(),
        driver: Some(types::Driver {
            name: "qemu".to_string(),
            driver_type: "raw".to_string(),
        }),
        source: Some(types::Source {
            file: Some(seed_iso_path),
            bridge: None,
        }),
        target: Some(types::Target {
            dev: "hdb".to_string(),
            bus: "sata".to_string(),
        }),
        readonly: Some(types::Empty {}),
    };

    let hd_disk = types::Disk {
        device: "disk".to_string(),
        disk_type: "file".to_string(),
        driver: Some(types::Driver {
            name: "qemu".to_string(),
            driver_type: "qcow2".to_string(),
        }),
        source: Some(types::Source {
            file: Some(disk_path),
            bridge: None,
        }),
        target: Some(types::Target {
            dev: "vda".to_string(),
            bus: "virtio".to_string(),
        }),
        readonly: None,
    };

    let interface = types::Interface {
        interface_type: "bridge".to_string(),
        source: Some(types::Source {
            file: None,
            bridge: Some("virbr0".to_string()),
        }),
        model: Some(types::Model {
            model_type: "virtio".to_string(),
        }),
    };

    let graphics = types::Graphics {
        graphics_type: "vnc".to_string(),
        port: "-1".to_string(),
        autoport: "yes".to_string(),
    };

    let console = types::Console {
        console_type: "pty".to_string(),
        target: Some(types::ConsoleTarget {
            type_: "serial".to_string(),
            port: "0".to_string(),
        }),
    };

    let serial = types::Serial {
        serial_type: "pty".to_string(),
        target: Some(types::SerialTarget {
            type_: "isa-serial".to_string(),
            port: "0".to_string(),
        }),
    };

    let devices = types::Devices {
        disk: vec![iso_disk, hd_disk],
        interface: Some(interface),
        graphics: Some(graphics),
        console: Some(console),
        serial: Some(serial),
    };

    let domain_config = types::DomainConfig {
        domain_type: "kvm".to_string(),
        name: name.to_string(),
        uuid: domain_uuid,
        os: Some(types::Os {
            os_type: types::OsType {
                arch: "x86_64".to_string(),
                machine: "pc-q35-6.2".to_string(),
                text: "hvm".to_string(),
            },
            boot: vec![
                types::Boot {
                    dev: "cdrom".to_string(),
                },
                types::Boot {
                    dev: "hd".to_string(),
                },
            ],
            //cmdline: Some("autoinstall ds=nocloud-net;s=https://polite-sunshine-3417ed.netlify.app/autoinstall.yml".to_string()),
        }),
        memory: Some(memory_config),
        current_memory: Some(current_memory_config),
        vcpu: Some(vcpu_config),
        devices: Some(devices),
    };

    let xml_element = helpers::struct_to_xml(&domain_config, "domain");

    xml_element
}

pub fn hash_password_sha512(password: &str) -> Result<String, sha_crypt::CryptError> {
    // Create params (choose rounds -- 10_000 is a reasonable default)
    let params = Sha512Params::new(10_000)?;
    // sha512_simple returns Result<String, CryptError>
    sha512_simple(password, &params)
}

pub fn create_seed_iso(name: &str, username: &str, password: &str) -> String {
    let hashed_password = hash_password_sha512(password).unwrap();

    println!("Username: {}", username);
    println!("Password: {}", password);
    println!("Hashed Password: {}", hashed_password);

    let user_data_yaml = format!(
        r#"#cloud-config
hostname: {}
locale: en_US.UTF-8
keyboard:
  layout: us

users:
  - name: {}
    sudo: "ALL=(ALL) NOPASSWD:ALL"
    lock_passwd: false
    passwd: $6$XiMjf17UrO/hnVZj$bJa4BRkVrdKwEDtEJPh4D3Xiw6LFu87LlaX4l9fbk7OVp2w5WmN9VTmQA6hQ0N3zegiXPXcfTVWIYo80Vqt9i.
    shell: /bin/bash
    groups: [sudo]
    gecos: User
    
ssh_pwauth: true
"#,
        name, username
    );

    let meta_data_yaml = format!(
        r#"instance-id: {}
local-hostname: {}
"#,
        name, name
    );

    // add required #cloud-config header to user_data
    //user_data_yaml = format!("#cloud-config\n{}", user_data_yaml);

    println!("User Data YAML:\n{}", user_data_yaml);
    println!("Meta Data YAML:\n{}", meta_data_yaml);

    let iso_path = format!("/var/lib/libvirt/images/{}-seed.iso", name);
    let iso_path_obj = Path::new(&iso_path);
    if let Some(parent) = iso_path_obj.parent() {
        if !parent.exists() {
            println!("Creating directory: {:?}", parent);
            Command::new("mkdir")
                .arg("-p")
                .arg(parent)
                .status()
                .expect("Failed to create parent directory.");
        }
    }

    let user_data_file = format!("/tmp/{}-user-data", name);
    let meta_data_file = format!("/tmp/{}-meta-data", name);

    std::fs::write(&user_data_file, user_data_yaml).expect("Unable to write user-data file");
    std::fs::write(&meta_data_file, meta_data_yaml).expect("Unable to write meta-data file");
    let output = Command::new("cloud-localds")
        .args(&[&iso_path, &user_data_file, &meta_data_file])
        .output()
        .expect("Failed to execute cloud-localds command");
    if output.status.success() {
        println!("Seed ISO created successfully at {}", &iso_path);
    } else {
        eprintln!(
            "Error creating seed ISO: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        std::process::exit(1);
    }

    // Clean up temporary files
    std::fs::remove_file(&user_data_file).expect("Unable to delete temporary user-data file");
    std::fs::remove_file(&meta_data_file).expect("Unable to delete temporary meta-data file");
    iso_path
}
