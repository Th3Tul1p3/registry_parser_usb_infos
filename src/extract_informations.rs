use crate::structure::*;
use crate::utility;
use nt_hive::*;
use std::str;
use utility::*;

pub fn get_vendor_product_version<'a>(
    root_key_node: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>,
    list: &mut Vec<UsbInfo>,
) {
    // get list of all subkeys
    let usbstor = root_key_node
        .subpath(CONTROLSET_ENUM_USBSTOR)
        .unwrap()
        .unwrap();
    let key_list = usbstor.subkeys().unwrap().unwrap();

    for key in key_list {
        let mut tmp: UsbInfo = UsbInfo::new();
        // get the raw string, the name of the key
        let string_device_class_id: String = name_to_string_keynode(key);

        let split_infos: Vec<&str> = string_device_class_id.split("&").collect::<Vec<&str>>();
        tmp.drive_type = split_infos.get(0).unwrap().to_string();
        tmp.vendor = split_infos.get(1).unwrap().to_string();
        tmp.product = split_infos.get(2).unwrap().to_string();
        tmp.version = split_infos.get(3).unwrap().to_string();

        // retrieve USB Unique serial number
        let path_to_device_class_id =
            [CONTROLSET_ENUM_USBSTOR, "\\", &string_device_class_id].join("");
        let unique_serial_number_folder = root_key_node
            .subpath(&path_to_device_class_id)
            .unwrap()
            .unwrap();
        let mut unique_serial_number_keys = unique_serial_number_folder.subkeys().unwrap().unwrap();
        get_usb_unique_serial_number(&mut unique_serial_number_keys, &mut tmp);

        // for an unknown reason there is an "&0" at the end of the USN. it's add again to get the timestamps
        let path_usn = [&path_to_device_class_id, "\\", &tmp.usn, "&0"].join("");
        get_all_timestamps(root_key_node, path_usn, &mut tmp);
        list.push(tmp);
    }
}

pub fn get_all_timestamps<'a>(
    root_key_node: &'a KeyNode<&Hive<&'a [u8]>, &'a [u8]>,
    path_to_get_usn: String,
    usbinfo: &mut UsbInfo,
) {
    // first install
    let first_install_path = [&path_to_get_usn, SUFFIX_FIRST_INSTALL].join("");
    let first_install_path_values = root_key_node.subpath(&first_install_path).unwrap().unwrap();
    usbinfo.first_install = extract_timestamp(&first_install_path_values);

    // Last Connected
    let last_connected_path = [&path_to_get_usn, SUFFIX_LAST_CONNECTED].join("");
    let last_connected_path_values = root_key_node
        .subpath(&last_connected_path)
        .unwrap()
        .unwrap();
    usbinfo.last_connected = extract_timestamp(&last_connected_path_values);

    // Last Removal
    let last_removal_path = [&path_to_get_usn, SUFFIX_LAST_REMOVED].join("");
    let last_removal_path_values = root_key_node.subpath(&last_removal_path).unwrap().unwrap();
    usbinfo.last_removed = extract_timestamp(&last_removal_path_values);
}

fn get_usb_unique_serial_number<'a>(
    registry: &mut SubKeyNodes<'a, &'a [u8]>,
    usbinfo: &mut UsbInfo,
) {
    for raw_usn in registry {
        let raw_string_usn: String = raw_usn
            .unwrap()
            .name()
            .unwrap()
            .to_string_checked()
            .unwrap();
        let string_usn = *raw_string_usn
            .split("&")
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap();
        usbinfo.usn = String::from_utf8(string_usn.as_bytes().to_vec()).unwrap();
    }
}

pub fn get_volume_name_drive_letter<'a>(
    root_key_node: &'a mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>,
    list: &mut Vec<UsbInfo>,
) {
    // get list of all subkeys
    let devices = root_key_node.subpath(MICROSOFT_WPD_DEVICES).unwrap();
    let device_subkey = devices.unwrap();
    let key_list = device_subkey.subkeys().unwrap().unwrap();

    for key in key_list {
        let raw_key = key.unwrap();
        let string_key: String = raw_key.name().unwrap().to_string_checked().unwrap();
        let split_infos: Vec<&str> = string_key.split("#").collect::<Vec<&str>>();

        let friendly_name = match raw_key
            .value("FriendlyName")
            .unwrap()
            .unwrap()
            .string_data()
        {
            Ok(friendly_name) => friendly_name,
            Err(_) => continue,
        };

        if split_infos.len() > 5 {
            let usn = split_infos.get(4).unwrap().to_string();
            let split_usn: Vec<&str> = usn.split("&").collect::<Vec<&str>>();
            match find_usn(split_usn.get(0).unwrap().to_string(), list) {
                None => {}
                Some(position) => {
                    list.get_mut(position)
                        .unwrap()
                        .friendly_name
                        .push(friendly_name);
                }
            }
        } else {
            // means that there is no usn in string
            //println!("Volume GUID: {}", split_infos.get(2).unwrap())
        }
    }
}

pub fn get_vid_pid<'a>(
    root_key_node: &'a mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>,
    list: &mut Vec<UsbInfo>,
) {
    // get list of all subkeys
    let usbstor = get_directory(root_key_node, CONTROLSET_ENUM_USB);
    let key_list = usbstor.subkeys().unwrap().unwrap();

    for key in key_list {
        let raw_vid_pid = key.unwrap();
        let string_vid_pid: String = raw_vid_pid.name().unwrap().to_string_checked().unwrap();
        let split_infos: Vec<&str> = string_vid_pid.split("&").collect::<Vec<&str>>();

        let mut vid_pid_subkeys = raw_vid_pid.subkeys().unwrap().unwrap();
        let raw_usn = vid_pid_subkeys.next().unwrap().unwrap();

        let mut pid = String::new();
        if !split_infos.get(0).unwrap().contains("ROOT_HUB") {
            pid = split_infos.get(1).unwrap().to_string();
        }

        match find_usn(raw_usn.name().unwrap().to_string(), list) {
            None => {
                let mut tmp = UsbInfo::new();
                tmp.pid = pid;
                tmp.usn = raw_usn.name().unwrap().to_string();
                tmp.vid = split_infos.get(0).unwrap().to_string();
                list.push(tmp);
            }
            Some(position) => {
                list.get_mut(position).unwrap().pid = pid;
                list.get_mut(position).unwrap().vid = split_infos.get(0).unwrap().to_string();
            }
        }
    }
}

pub fn get_volume_guid<'a>(
    root_key_node: &'a mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>,
    list: &mut Vec<UsbInfo>,
) {
    // get list of all subkeys
    let mounteddevices = get_directory(root_key_node, MOUNTED_DEVICES);
    let key_values_list = mounteddevices.values().unwrap().unwrap();

    for key_value in key_values_list {
        let raw_s = key_value.unwrap();
        let key_value_name = raw_s.name().unwrap().to_string();

        let mut usn = String::new();
        let binary_data = raw_s.data().unwrap().into_vec().unwrap();
        match str::from_utf8(&binary_data) {
            Ok(string_data) => {
                let split_infos: Vec<&str> = string_data.split("&").collect::<Vec<&str>>();
                if split_infos.len() > 4 {
                    let extract_usn_version = split_infos
                        .get(3)
                        .unwrap()
                        .split("#")
                        .collect::<Vec<&str>>();
                    for c in extract_usn_version.get(1).unwrap().as_bytes().iter() {
                        if *c != 0 {
                            usn.push(*c as char);
                        }
                    }
                }
            }
            Err(_err) => unsafe {
                if str::from_utf8_unchecked(&binary_data).contains("DMIO:ID:") {
                    println!("Signature of GPT partition, start with DMIO...");
                }
            },
        };
        match find_usn(usn.clone(), list) {
            None => {}
            Some(position) => {
                if key_value_name.contains("DosDevices") {
                    list.get_mut(position)
                        .unwrap()
                        .friendly_name
                        .push(key_value_name[key_value_name.len() - 3..].to_string());
                } else {
                    // extract GUID
                    list.get_mut(position).unwrap().guid = match key_value_name.find('{') {
                        None => continue,
                        Some(pos) => key_value_name[pos..pos + 38].to_string(),
                    };
                }
            }
        }
    }
}
