use nt_hive::*;
use std::io;
mod utility;
use utility::*;
mod extract_informations;
use extract_informations::*;
mod structure;
use structure::UsbInfo;

fn main() -> io::Result<()> {
    let mut list_usb_keys_infos: Vec<UsbInfo> = vec![];
    let system_buffer = utility::get_registry_root("system");
    let system_hive: Hive<&[u8]> = Hive::new(system_buffer.as_ref()).unwrap();
    let system_root_key_node: KeyNode<&Hive<&[u8]>, &[u8]> = system_hive.root_key_node().unwrap();

    let software_buffer = utility::get_registry_root("software");
    let software_hive: Hive<&[u8]> = Hive::new(software_buffer.as_ref()).unwrap();
    let mut software_root_key_node = software_hive.root_key_node().unwrap();

    separator();
    println!("-- Get Vendor, Product, Version and unique serial number");
    get_vendor_product_version(&mut system_root_key_node.clone(), &mut list_usb_keys_infos);

    println!("-- Get Vendor-ID (VID) and Product-ID (PID)");
    get_vid_pid(&mut system_root_key_node.clone(), &mut list_usb_keys_infos);

    println!("-- Get Volume GUIDs");
    get_volume_guid(&mut system_root_key_node.clone());

    println!("-- Get Drive letter and Volume Name");
    get_volume_name_drive_letter(&mut software_root_key_node);

    println!("-- Get Volume Serial Number");
    println!("-- Get User that used USB");

    for usb in list_usb_keys_infos {
        println!("{}", usb);
    }

    Ok(())
}
