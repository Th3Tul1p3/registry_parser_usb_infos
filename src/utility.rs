
use std::fs::File;
use std::io::Read;

pub static SUFFIX_FIRST_INSTALL: &str = "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0064";
pub static _SUFFIX_LAST_CONNECTED: &str = "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0066";
pub static _SUFFIX_LAST_REMOVED: &str = "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0067";
pub static _SUFFIX_FIRST_INSTALL_DATE: &str =
    "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0065";
    pub static MICROSOFT_WPD_DEVICES: &str = "Microsoft\\Windows Portable Devices\\Devices";
    pub static CONTROLSET_ENUM_USBSTOR: &str = "ControlSet001\\Enum\\USBSTOR";
    pub static MOUNTED_DEVICES: &str = "MountedDevices";
    pub static CONTROLSET_ENUM_USB: &str = "ControlSet001\\Enum\\USB";

pub fn get_registry_root<'a>(filename: &str) -> Vec<u8> {
    let mut system_buffer: Vec<u8> = Vec::new();
    File::open(filename.to_string())
        .unwrap()
        .read_to_end(&mut system_buffer)
        .unwrap();
    return system_buffer;
}