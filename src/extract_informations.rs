use crate::utility;
use nt_hive::Hive;
use nt_hive::KeyNode;
use nt_hive::SubKeyNodes;
use std::str;
use utility::get_directory;

pub fn get_vendor_product_version<'a>(root_key_node: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    // get list of all subkeys
    let usbstor = root_key_node
        .subpath(utility::CONTROLSET_ENUM_USBSTOR)
        .unwrap()
        .unwrap();
    let key_list = usbstor.subkeys().unwrap().unwrap();

    for key in key_list {
        // get the raw string, the name of the key
        let string_device_class_id: String = utility::name_to_string_keynode(key);

        // split
        let split_infos: Vec<&str> = string_device_class_id.split("&").collect::<Vec<&str>>();
        println!("Type: {}", split_infos.get(0).unwrap());
        println!("Vendor: {}", split_infos.get(1).unwrap());
        println!("Product: {}", split_infos.get(2).unwrap());
        println!("Version: {}", split_infos.get(3).unwrap());

        // retrieve USB Unique serial number
        let path = [
            utility::CONTROLSET_ENUM_USBSTOR,
            "\\",
            &string_device_class_id,
        ]
        .join("");
        let unique_serial_number_folder = root_key_node.subpath(&path).unwrap().unwrap();
        let mut unique_serial_number_keys = unique_serial_number_folder.subkeys().unwrap().unwrap();
        get_usb_unique_serial_number(&mut unique_serial_number_keys);

        println!()
    }
    utility::separator();
}

fn get_usb_unique_serial_number<'a>(registry: &mut SubKeyNodes<'a, &'a [u8]>) -> Box<String> {
    let mut box_string_usn = Box::new("".to_string());
    for raw_usn in registry {
        let string_usn: String = raw_usn
            .unwrap()
            .name()
            .unwrap()
            .to_string_checked()
            .unwrap();
        box_string_usn = Box::new(string_usn);
        println!("Unique serial number: {}", box_string_usn);
    }
    return box_string_usn;
}

pub fn get_volume_name_drive_letter<'a>(root_key_node: &'a mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    // get list of all subkeys
    let devices = utility::get_directory(root_key_node, utility::MICROSOFT_WPD_DEVICES);
    let key_list = devices.subkeys().unwrap().unwrap();

    for key in key_list {
        let raw_key = key.unwrap();
        let string_key: String = raw_key.name().unwrap().to_string_checked().unwrap();
        let split_infos: Vec<&str> = string_key.split("#").collect::<Vec<&str>>();
        if split_infos.len() > 5 {
            println!("Unique serial number: {}", split_infos.get(4).unwrap());
        } else {
            println!("Volume GUID: {}", split_infos.get(2).unwrap())
        }

        let key_value = raw_key.value("FriendlyName").unwrap().unwrap();
        let multi_sz_data = key_value.string_data();
        if let Ok(vec) = multi_sz_data {
            println!("Friendly name : {:?}", vec);
        }

        println!()
    }
    utility::separator();
}

pub fn get_vid_pid<'a>(root_key_node: &'a mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    // get list of all subkeys
    let usbstor = utility::get_directory(root_key_node, utility::CONTROLSET_ENUM_USB);
    let key_list = usbstor.subkeys().unwrap().unwrap();

    for key in key_list {
        let raw_vid_pid = key.unwrap();
        let string_vid_pid: String = raw_vid_pid.name().unwrap().to_string_checked().unwrap();
        let split_infos: Vec<&str> = string_vid_pid.split("&").collect::<Vec<&str>>();
        println!("VID: {}", split_infos.get(0).unwrap());
        if !split_infos.get(0).unwrap().contains("ROOT_HUB") {
            println!("PID: {}", split_infos.get(1).unwrap());
        }

        let mut vid_pid_subkeys = raw_vid_pid.subkeys().unwrap().unwrap();
        let raw_usn = vid_pid_subkeys.next().unwrap().unwrap();
        println!("Unique serial number: {}", raw_usn.name().unwrap());
        println!()
    }
    utility::separator();
}

pub fn get_volume_guid<'a>(root_key_node: &'a mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    //Seems that the parsing of MountedDevices does not work
    // get list of all subkeys
    let mounteddevices = get_directory(root_key_node, utility::MOUNTED_DEVICES);
    let key_values_list = mounteddevices.values().unwrap().unwrap();

    for key_value in key_values_list {
        let raw_s = key_value.unwrap();
        println!("{}", raw_s.name().unwrap());
        let binary_data = raw_s.data().unwrap().into_vec().unwrap();
        match str::from_utf8(&binary_data) {
            Ok(string_data) => {
                let split_infos: Vec<&str> = string_data.split("&").collect::<Vec<&str>>();
                if split_infos.len() > 4 {
                    let extract_type = split_infos
                        .get(0)
                        .unwrap()
                        .split("#")
                        .collect::<Vec<&str>>();
                    let extract_usn_version = split_infos
                        .get(3)
                        .unwrap()
                        .split("#")
                        .collect::<Vec<&str>>();

                    println!("Type: {}", extract_type.get(1).unwrap());
                    println!("Vendor: {}", split_infos.get(1).unwrap());
                    println!("Product: {}", split_infos.get(2).unwrap());
                    println!("Version: {}", extract_usn_version.get(0).unwrap());
                    println!("Unique serial number: {}", extract_usn_version.get(1).unwrap());
                }
            }
            Err(_err) => unsafe {
                if str::from_utf8_unchecked(&binary_data).contains("DMIO:ID:") {
                    println!("Signature of GPT partition, start with DMIO...");
                } else {
                    println!("Impossible to decode {{{:?}}}", binary_data);
                }
            },
        };
        println!();
    }
    utility::separator();
}
