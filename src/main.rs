use nt_hive::Hive;
use nt_hive::KeyNode;
use std::io;
mod utility;
use utility::*;
mod extract_informations;
use extract_informations::*;

fn main() -> io::Result<()> {
    let system_buffer = utility::get_registry_root("system");
    let system_hive: Hive<&[u8]> = Hive::new(system_buffer.as_ref()).unwrap();
    let mut system_root_key_node: KeyNode<&Hive<&[u8]>, &[u8]> =
        system_hive.root_key_node().unwrap();

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

    get_all_timestamps(&mut system_root_key_node);

    Ok(())
}

fn get_all_timestamps<'a>(root_key_node: &'a mut KeyNode<&'a Hive<&'a [u8]>, &'a [u8]>) {
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

        let unique_serial_number_folder = prepare_path(
            &utility::CONTROLSET_ENUM_USBSTOR,
            &string_device_class_id,
            "",
            root_key_node,
        );

        let mut unique_serial_number_keys = unique_serial_number_folder.subkeys().unwrap().unwrap();
        let unique_serial_number_key = unique_serial_number_keys.next().unwrap();
        let extract_usn: String = utility::name_to_string_keynode(unique_serial_number_key);
        println!("{}", string_device_class_id);

        // first install
        let mut first_install_path = prepare_path(
            &path_to_get_usn,
            &extract_usn,
            utility::SUFFIX_FIRST_INSTALL,
            root_key_node,
        );
        print_timestamp(&mut first_install_path, "First install (UTC):");

        // Last Connected
        let mut last_connected_path = prepare_path(
            &path_to_get_usn,
            &extract_usn,
            utility::SUFFIX_LAST_CONNECTED,
            root_key_node,
        );
        print_timestamp(&mut last_connected_path, "Last Connected (UTC):");

        // Last Removal
        let mut last_removal_path = prepare_path(
            &path_to_get_usn,
            &extract_usn,
            utility::SUFFIX_LAST_REMOVED,
            root_key_node,
        );
        print_timestamp(&mut last_removal_path, "Last Removal (UTC):");

        println!()
    }
    separator();
}

fn prepare_path<'a>(
    path_to_get_usn: &str,
    extract_usn: &str,
    suffix: &str,
    root_key_node: &'a KeyNode<&Hive<&'a [u8]>, &'a [u8]>,
) -> KeyNode<&'a Hive<&'a [u8]>, &'a [u8]> {
    let last_removal_install = [&path_to_get_usn, "\\", &extract_usn, suffix].join("");
    root_key_node
        .subpath(&last_removal_install)
        .unwrap()
        .unwrap()
}

fn print_timestamp<'a>(path: &mut KeyNode<&Hive<&'a [u8]>, &'a [u8]>, message: &str) {
    let mut values = path.values().unwrap().unwrap();
    let raw_value = values.next().unwrap().unwrap();
    let raw_nanos_value = raw_value.data().unwrap().into_vec().unwrap();
    let timestamp_part = split_iso_timestamp(utility::rawvalue_to_timestamp(raw_nanos_value));

    println!(
        "{} {} {}",
        message,
        timestamp_part.get(0).unwrap(),
        timestamp_part.get(1).unwrap()
    );
}
