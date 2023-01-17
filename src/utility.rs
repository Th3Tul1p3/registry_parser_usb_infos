use nt_hive::Hive;
use nt_hive::KeyNode;
use nt_hive::NtHiveError;
use std::fs::File;
use std::io::Read;

pub static SUFFIX_FIRST_INSTALL: &str =
    "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0064";
pub static _SUFFIX_LAST_CONNECTED: &str =
    "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0066";
pub static _SUFFIX_LAST_REMOVED: &str =
    "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0067";
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

pub fn get_directory<'a>(
    root_key_node: &'a mut KeyNode<&'a Hive<&'a [u8]>, &'a [u8]>,
    path: &str,
) -> KeyNode<&'a Hive<&'a [u8]>, &'a [u8]> {
    let usbstor = root_key_node.subpath(path).unwrap().unwrap();
    return usbstor;
}

pub fn keynode_name_to_string<'a>(
    param: Result<KeyNode<&Hive<&'a [u8]>, &'a [u8]>, NtHiveError>,
) -> String {
    let raw_device_class_id = param.unwrap();
    raw_device_class_id
        .name()
        .unwrap()
        .to_string_checked()
        .unwrap()
}

pub fn separator() {
    println!("------------------------------------------------------------------------------------------------------------")
}
