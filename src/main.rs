use clap::{Parser, Subcommand};
use vm::{boot_vm, create_vm, delete_vm, list_vms, restart_vm, shutdown_vm, vm_info};

pub mod helpers;
pub mod vm;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Create {
        /// Name of the VM
        #[arg(short, long)]
        name: String,

        /// Username for the VM
        #[arg(short, long, default_value = "junior")]
        username: String,

        /// Password for the VM
        #[arg(short, long, default_value = "123456789")]
        password: String,

        /// Memory to allocate to the VM
        #[arg(short, long, default_value = "2048")]
        memory: u64,

        /// Number of vCPUs to allocate to the VM
        #[arg(short, long, default_value = "3")]
        vcpus: u8,

        /// Disk size in GB
        #[arg(short, long, default_value = "10")]
        disk_size: u64,
    },
    List,
    Delete {
        /// Name of the VM
        #[arg(short, long)]
        name: String,
    },
    Boot {
        /// Name of the VM
        #[arg(short, long)]
        name: String,
    },
    Shutdown {
        /// Name of the VM
        #[arg(short, long)]
        name: String,
    },
    Restart {
        /// Name of the VM
        #[arg(short, long)]
        name: String,
    },
    VMInfo {
        /// Name of the VM
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Boot { name } => {
            boot_vm(&name);
        }
        Commands::Create {
            name,
            memory,
            vcpus,
            disk_size,
            username,
            password,
        } => {
            create_vm(&name, &username, &password, memory, vcpus, disk_size);
        }
        Commands::Delete { name } => {
            delete_vm(&name);
        }
        Commands::List => {
            list_vms();
        }
        Commands::Restart { name } => {
            restart_vm(&name);
        }
        Commands::Shutdown { name } => {
            shutdown_vm(&name);
        }
        Commands::VMInfo { name } => {
            vm_info(&name);
        }
    }
    ();
}
