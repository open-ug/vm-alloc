use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DomainConfig {
    #[serde(rename = "@type")]
    pub domain_type: String,

    pub name: String,
    pub uuid: String,

    pub os: Option<Os>,

    pub memory: Option<Memory>,

    #[serde(rename = "currentMemory")]
    pub current_memory: Option<Memory>,

    pub vcpu: Option<Vcpu>,

    pub devices: Option<Devices>,
}

#[derive(Serialize, Deserialize)]
pub struct Os {
    #[serde(rename = "type")]
    pub os_type: OsType,

    pub boot: Vec<Boot>,
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub cmdline: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct OsType {
    #[serde(rename = "@arch")]
    pub arch: String,

    #[serde(rename = "@machine")]
    pub machine: String,

    #[serde(rename = "#text")]
    pub text: String,
}

#[derive(Serialize, Deserialize)]
pub struct Boot {
    #[serde(rename = "@dev")]
    pub dev: String,
}

#[derive(Serialize, Deserialize)]
pub struct Memory {
    #[serde(rename = "@unit")]
    pub unit: String,

    #[serde(rename = "#text")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Vcpu {
    #[serde(rename = "@placement")]
    pub placement: String,
    #[serde(rename = "#text")]
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct Devices {
    pub disk: Vec<Disk>,
    pub interface: Option<Interface>,
    pub graphics: Option<Graphics>,
    pub console: Option<Console>,
    pub serial: Option<Serial>,
}

#[derive(Serialize, Deserialize)]
pub struct Console {
    #[serde(rename = "@type")]
    pub console_type: String,
    pub target: Option<ConsoleTarget>,
}

#[derive(Serialize, Deserialize)]
pub struct Serial {
    #[serde(rename = "@type")]
    pub serial_type: String,
    pub target: Option<SerialTarget>,
}

#[derive(Serialize, Deserialize)]
pub struct ConsoleTarget {
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@port")]
    pub port: String,
}

#[derive(Serialize, Deserialize)]
pub struct SerialTarget {
    #[serde(rename = "@type")]
    pub type_: String,
    #[serde(rename = "@port")]
    pub port: String,
}

#[derive(Serialize, Deserialize)]
pub struct Disk {
    #[serde(rename = "@type")]
    pub disk_type: String,
    #[serde(rename = "@device")]
    pub device: String,
    pub driver: Option<Driver>,
    pub source: Option<Source>,
    pub target: Option<Target>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct Driver {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub driver_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Source {
    #[serde(rename = "@file", skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,
    #[serde(rename = "@bridge", skip_serializing_if = "Option::is_none")]
    pub bridge: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Target {
    #[serde(rename = "@dev")]
    pub dev: String,
    #[serde(rename = "@bus")]
    pub bus: String,
}

#[derive(Serialize, Deserialize)]
pub struct Interface {
    #[serde(rename = "@type")]
    pub interface_type: String,
    pub source: Option<Source>,
    pub model: Option<Model>,
}

#[derive(Serialize, Deserialize)]
pub struct Model {
    #[serde(rename = "@type")]
    pub model_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Graphics {
    #[serde(rename = "@type")]
    pub graphics_type: String,
    #[serde(rename = "@port")]
    pub port: String,
    #[serde(rename = "@autoport")]
    pub autoport: String,
}

/// For <readonly/> empty tags
#[derive(Serialize, Deserialize)]
pub struct Empty {}

// Cloud init related structs, meant to be serialized to user-data and meta-data files
#[derive(Serialize, Deserialize)]
pub struct CloudInitUserData {
    pub hostname: String,
    pub locale: String,
    pub keyboard: Keyboard,
    pub ssh: Ssh,
    pub lock_passwd: bool,
    pub users: Vec<CloudInitUser>,
    pub ssh_pwauth: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CloudInitUser {
    pub name: String,
    pub passwd: String,
    pub gecos: String,
    pub groups: Vec<String>,
    pub sudo: String,
    pub shell: String,
}

#[derive(Serialize, Deserialize)]
pub struct Keyboard {
    pub layout: String,
}

#[derive(Serialize, Deserialize)]
pub struct Ssh {
    pub install_server: bool,
}

#[derive(Serialize, Deserialize)]
pub struct CloudInitMetaData {
    pub instance_id: String,
    pub local_hostname: String,
}
