use nt_hive::Hive;
use nt_hive::KeyNode;
use std::io;
mod utility;
use utility::*;
mod extract_informations;
use datetime::ISO;
use extract_informations::*;

fn main() -> io::Result<()> {
    let system_buffer = utility::get_registry_root("system");
    let system_hive: Hive<&[u8]> = Hive::new(system_buffer.as_ref()).unwrap();
    let mut system_root_key_node = system_hive.root_key_node().unwrap();

    let software_buffer = utility::get_registry_root("software");
    let software_hive: Hive<&[u8]> = Hive::new(software_buffer.as_ref()).unwrap();
    let mut software_root_key_node = software_hive.root_key_node().unwrap();

    separator();
    println!("-- Get Vendor, Product, Version and unique serial number");
    get_vendor_product_version(&mut system_root_key_node.clone());

    println!("-- Get Vendor-ID (VID) and Product-ID (PID)");
    get_vid_pid(&mut system_root_key_node.clone());

    println!("-- Get Volume GUIDs");
    get_volume_guid(&mut system_root_key_node.clone());

    println!("-- Get Drive letter and Volume Name");
    get_volume_name_drive_letter(&mut software_root_key_node);

    println!("-- Get Volume Serial Number");
    println!("-- Get User that used USB");
    println!("-- Get First time device was connected");
    get_timestamps(&mut system_root_key_node);
    println!("-- Get First time device was connected after reboot");
    println!("-- Get Last time connected");
    println!("-- Get Time device was removed");
    Ok(())
}

fn get_timestamps<'a>(root_key_node: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>) {
    // get list of all subkeys
    let usbstor = root_key_node
        .subpath(utility::CONTROLSET_ENUM_USBSTOR)
        .unwrap()
        .unwrap();
    let key_list = usbstor.subkeys().unwrap().unwrap();

    for key in key_list {
        // get the raw string, the name of the key
        let string_device_class_id: String = utility::name_to_string_keynode(key);

        // retrieve USB Unique serial number
        let path_to_get_usn = [
            utility::CONTROLSET_ENUM_USBSTOR,
            "\\",
            &string_device_class_id,
        ]
        .join("");
        let unique_serial_number_folder = root_key_node.subpath(&path_to_get_usn).unwrap().unwrap();
        let mut unique_serial_number_keys = unique_serial_number_folder.subkeys().unwrap().unwrap();
        let unique_serial_number_key = unique_serial_number_keys.next().unwrap();
        let extract_usn: String = utility::name_to_string_keynode(unique_serial_number_key);
        let path_first_install = [
            &path_to_get_usn,
            "\\",
            &extract_usn,
            utility::SUFFIX_FIRST_INSTALL,
        ]
        .join("");
        let first_install_path = root_key_node.subpath(&path_first_install).unwrap().unwrap();
        println!("{}", path_first_install);
        let mut first_install_values = first_install_path.values().unwrap().unwrap();

        let raw_value = first_install_values.next().unwrap().unwrap();
        let raw_nanos_value = raw_value.data().unwrap().into_vec().unwrap();

        println!(
            "{}",
            utility::rawvalue_to_timestamp(raw_nanos_value)
                .iso()
                .to_string()
        );

        println!()
    }
    separator();
}
