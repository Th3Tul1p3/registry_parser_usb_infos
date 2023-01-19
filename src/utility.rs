use datetime::LocalDate;
use datetime::LocalDateTime;
use datetime::LocalTime;
use datetime::Month;
use datetime::ISO;
use nt_hive::Hive;
use nt_hive::KeyNode;
use nt_hive::NtHiveError;
use std::convert::TryInto;
use std::fs::File;
use std::io::Read;
use std::time::Duration;

pub static SUFFIX_FIRST_INSTALL: &str =
    "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0064";
pub static SUFFIX_LAST_CONNECTED: &str =
    "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0066";
pub static SUFFIX_LAST_REMOVED: &str = "\\Properties\\{83da6326-97a6-4088-9453-a1923f573b29}\\0067";
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

pub fn name_to_string_keynode<'a>(
    param: Result<KeyNode<&Hive<&'a [u8]>, &'a [u8]>, NtHiveError>,
) -> String {
    let raw = param.unwrap();
    raw.name().unwrap().to_string_checked().unwrap()
}

pub fn separator() {
    println!("------------------------------------------------------------------------------------------------------------")
}

pub fn rawvalue_to_timestamp(tmp: Vec<u8>) -> LocalDateTime {
    let bytes_to_nanos = u64::from_le_bytes(tmp.try_into().unwrap()) * 100;
    let nanos_to_secs: i64 = Duration::from_nanos(bytes_to_nanos)
        .as_secs()
        .try_into()
        .unwrap();
    let windows_base_date = LocalDate::ymd(1601, Month::January, 1).unwrap();
    let hour: i8 = 0;
    let minute: i8 = 0;
    let windows_base_time = LocalTime::hm(hour, minute).unwrap();
    let windows_base_timestamp = LocalDateTime::new(windows_base_date, windows_base_time);
    windows_base_timestamp.add_seconds(nanos_to_secs)
}

pub fn split_iso_timestamp<'a>(iso_timestamp: LocalDateTime) -> Vec<String> {
    let mut string_vec: Vec<String> = Vec::new();
    iso_timestamp
        .iso()
        .to_string()
        .split("T")
        .for_each(|x| string_vec.push(x.to_string()));
    return string_vec;
}

pub fn print_timestamp<'a>(path: &KeyNode<&Hive<&'a [u8]>, &'a [u8]>, message: &str) {
    let mut values = path.values().unwrap().unwrap();
    let raw_value = values.next().unwrap().unwrap();
    let raw_nanos_value = raw_value.data().unwrap().into_vec().unwrap();
    let timestamp_part = split_iso_timestamp(rawvalue_to_timestamp(raw_nanos_value));

    println!(
        "{} {} {}",
        message,
        timestamp_part.get(0).unwrap(),
        timestamp_part.get(1).unwrap()
    );
}
