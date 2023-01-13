use nt_hive::Hive;
use nt_hive::KeyNode;
use nt_hive::SubKeyNodes;
use std::fs::File;
use std::io;
use std::io::Read;
use std::str;

fn main() -> io::Result<()> {
    let system_buffer = get_registry_root("system");
    let system_hive: Hive<&[u8]> = Hive::new(system_buffer.as_ref()).unwrap();
    let mut system_root_key_node = system_hive.root_key_node().unwrap();

    let software_buffer = get_registry_root("software");
    let software_hive: Hive<&[u8]> = Hive::new(software_buffer.as_ref()).unwrap();
    let mut software_root_key_node = software_hive.root_key_node().unwrap();

    println!("-- Get Vendor, Product, Version and unique serial number");
    get_vendor_product_version(&mut system_root_key_node);

    println!("-- Get Vendor-ID (VID) and Product-ID (PID)");
    get_vid_pid(&mut system_root_key_node);

    println!("-- Get Volume GUIDs");
    get_volume_guid(&mut system_root_key_node);

    println!("-- Get Drive letter and Volume Name");
    get_volume_name_drive_letter(&mut software_root_key_node);

    println!("-- Get Volume Serial Number");
    println!("-- Get User that used USB");
    println!("-- Get First time device was connected");
    println!("-- Get First time device was connected after reboot");
    println!("-- Get Last time connected");
    println!("-- Get Time device was removed");
    Ok(())
}

fn get_registry_root<'a>(filename: &str) -> Vec<u8> {
    let mut system_buffer: Vec<u8> = Vec::new();
    File::open(filename.to_string())
        .unwrap()
        .read_to_end(&mut system_buffer)
        .unwrap();
    return system_buffer;
}

fn get_volume_name_drive_letter<'a>(root_key_node: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    let devices = root_key_node
        .subpath("Microsoft\\Windows Portable Devices\\Devices")
        .unwrap()
        .unwrap();
    let key_list = devices.subkeys().unwrap().unwrap();
    for key in key_list {
        let raw_key = key.unwrap();
        let string_key: String = raw_key.name().unwrap().to_string_checked().unwrap();
        let split_infos: Vec<&str> = string_key.split("#").collect::<Vec<&str>>();
        if split_infos.len() > 5 {
            println!("USN: {}", split_infos.get(4).unwrap());
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
}

fn get_volume_guid<'a>(root_key_node: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    //Seems that the parsing of MountedDevices does not work
    let mounteddevices = root_key_node.subpath("MountedDevices").unwrap().unwrap();
    let key_values_list = mounteddevices.values().unwrap().unwrap();
    for s in key_values_list {
        let raw_s = s.unwrap();
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
                    println!("USN: {}", extract_usn_version.get(1).unwrap());
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
}

fn get_vid_pid<'a>(root_key_node: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    let usbstor = root_key_node
        .subpath("ControlSet001\\Enum\\USB")
        .unwrap()
        .unwrap();
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
        println!("USN: {}", raw_usn.name().unwrap());
        println!()
    }
}

fn get_vendor_product_version<'a>(root_key_node: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    let usbstor = root_key_node
        .subpath("ControlSet001\\Enum\\USBSTOR")
        .unwrap()
        .unwrap();
    let key_list = usbstor.subkeys().unwrap().unwrap();
    for key in key_list {
        let raw_device_class_id = key.unwrap();
        let string_device_class_id: String = raw_device_class_id
            .name()
            .unwrap()
            .to_string_checked()
            .unwrap();
        let split_infos: Vec<&str> = string_device_class_id.split("&").collect::<Vec<&str>>();
        println!("Type: {}", split_infos.get(0).unwrap());
        println!("Vendor: {}", split_infos.get(1).unwrap());
        println!("Product: {}", split_infos.get(2).unwrap());
        println!("Version: {}", split_infos.get(3).unwrap());

        // retrieve USB Unique serial number
        let s = ["ControlSet001\\Enum\\USBSTOR\\", &string_device_class_id].join("");
        let unique_serial_number_folder = root_key_node.subpath(&s).unwrap().unwrap();
        let mut unique_serial_number_keys = unique_serial_number_folder.subkeys().unwrap().unwrap();
        get_usb_unique_serial_number(&mut unique_serial_number_keys);

        println!()
    }
}

fn get_usb_unique_serial_number<'a>(registry: &mut SubKeyNodes<&'a [u8]>) {
    for key in registry {
        let usn = key.unwrap();
        let usn_string = usn.name().unwrap();
        println!("Unique serial number: {}", usn_string);
    }
}
