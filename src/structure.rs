use std::fmt;

pub struct UsbInfo {
    pub drive_type: String,
    pub vendor: String,
    pub product: String,
    pub version: String,
    pub usn: String,
    pub first_install: String,
    pub last_connected: String,
    pub last_removed: String,
    pub pid: String,
    pub vid: String,
    pub guid: String,
}

impl fmt::Display for UsbInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Type: \t\t\t{}
Vendor: \t\t{}
Product: \t\t{}
Version: \t\t{}
Unique serial number: \t{}
First install (UTC): \t{}
Last Connected (UTC): \t{}
Last Removal (UTC): \t{}
PID: \t\t\t{}
VID: \t\t\t{}
GUID: \t\t\t{}\n",
            self.drive_type,
            self.vendor,
            self.product,
            self.version,
            self.usn,
            self.first_install,
            self.last_connected,
            self.last_removed,
            self.pid,
            self.vid,
            self.guid,
        )
    }
}

impl UsbInfo {
    pub fn new() -> Self {
        UsbInfo {
            drive_type: "".to_string(),
            vendor: "".to_string(),
            product: "".to_string(),
            version: "".to_string(),
            usn: "".to_string(),
            first_install: "".to_string(),
            last_connected: "".to_string(),
            last_removed: "".to_string(),
            pid: "".to_string(),
            vid: "".to_string(),
            guid: "".to_string(),
        }
    }
}

pub fn find_usn(usn: String, vector: &Vec<UsbInfo>) -> Option<usize> {
    vector.iter().position(|s|  s.usn == usn
    )
}
