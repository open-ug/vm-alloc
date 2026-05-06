# vm-alloc

**vm-alloc** is a command-line tool for spinning up and managing Virtual Machines (VMs) in cloud-like environments.
It is built on top of *libvirt*, *KVM*, and *QEMU*, and uses cloud images (primarily Ubuntu) for quick provisioning.

This project is intended for *research and exploration of virtualization in cloud environments*. It is not intended for production use.

## Requirements

- Linux host with virtualization support.
- Installed dependencies:
  - `libvirt`
  - `qemu`
  - `firecracker`
  - `virt-install`
  - `cloud-utils` (for `cloud-init`)
- Ubuntu cloud images.

## Usage

1. Clone this repository

```sh
git clone https://github.com/open-ug/vm-alloc.git
```

2. Install and Setup dependencies

```sh
sudo apt update

# Install QEMU, libvirt etc
sudo apt install -y qemu-kvm libvirt-daemon-system libvirt-clients virtinst bridge-utils cloud-utils

sudo systemctl enable --now libvirtd

sudo usermod -aG libvirt $(whoami)

newgrp libvirt
```

To install Firecraker, follow the [Getting Started with Firecracker Guide](https://github.com/firecracker-microvm/firecracker/blob/main/docs/getting-started.md)

3. Run the `init` command

```sh
cargo run init
```

4. Then Run the program

```sh
$ cargo run


Usage: vmprov <COMMAND>

Commands:
  init
  create    
  list      
  delete    
  boot      
  shutdown  
  restart   
  vm-info   
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

> Note: Different branches on this project explore different implementations of this program as follows
> 
> - The `main` branch explores an implementation using `libvirt` and `QEMU`
> - The `qemu-backend` branch explores and implementation using `QEMU` with the QEMU Monitor Protocol with cloud images
> - The `firecracker-backend` branch explores and implementation using AWS Firecracker with MicroVMs

## Disclaimer

This project is experimental and should not be used in production.
It is provided as-is for research purposes.

