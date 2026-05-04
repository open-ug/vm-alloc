use std::io::{Read, Write};
use std::os::unix::net::UnixStream;
use std::process::{Command, Stdio};

pub fn start_vm(disk: &str, seed: &str, qmp_socket: &str) {
    let child = Command::new("qemu-system-x86_64")
        .args([
            "-m",
            "2048",
            "-smp",
            "2",
            "-enable-kvm",
            "-cpu",
            "host",
            "-nographic",
            // OS disk
            "-drive",
            &format!("file={},format=qcow2,if=virtio", disk),
            // cloud-init seed
            "-drive",
            &format!("file={},format=raw,media=cdrom", seed),
            // networking
            "-netdev",
            "user,id=net0,hostfwd=tcp::2222-:22",
            "-device",
            "virtio-net-pci,netdev=net0",
            // QMP socket
            "-qmp",
            &format!("unix:{},server,nowait", qmp_socket),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to start QEMU");

    println!("Started VM with PID: {}", child.id());
}

pub fn connect_qmp(path: &str) -> UnixStream {
    let mut stream: UnixStream;

    loop {
        match UnixStream::connect(path) {
            Ok(s) => {
                stream = s;
                break;
            }
            Err(_) => {
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
        }
    }

    println!("Connected to QMP");
    stream
}

pub fn init_qmp(stream: &mut UnixStream) {
    let mut buffer = [0; 1024];

    // Read greeting
    let _ = stream.read(&mut buffer);

    // Send capabilities command
    let cmd = r#"{ "execute": "qmp_capabilities" }"#;
    stream.write_all(cmd.as_bytes()).unwrap();
}
