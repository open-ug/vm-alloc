# vm-alloc

**vm-alloc** is a command-line tool for spinning up and managing Virtual Machines (VMs) in cloud-like environments.
It is built on top of *libvirt*, *KVM*, and *QEMU*, and uses cloud images (primarily Ubuntu) for quick provisioning.

This project is intended for *research and exploration of virtualization in cloud environments*. It is not intended for production use.

## Requirements

- Linux host with virtualization support.
- Installed dependencies:
  - `libvirt`
  - `qemu`
  - `virt-install`
  - `cloud-utils` (for `cloud-init`)
- Ubuntu cloud images.

## Usage

```sh
Usage: vmprov <COMMAND>

Commands:
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

## Disclaimer

This project is experimental and should not be used in production.
It is provided as-is for research purposes.

## Author

Created by [Beingana Jim Junior](https://jim-junior.github.io/).
