pub struct Save {
    pub(crate) trainer_name: String,
    pub(crate) playtime_hours: u8 //in seconds, biggest number should be like 4.3 mill, biggest available time (999:95) is like... 3.6 mill, maybe a little more. 3.7 max
}

// Bytes from the save are not utf8... or i think so. The numbers need to be lower at least.
// The save's name should be RED, so have to finish this function
pub fn convert_trainer_name(bytes: [u8]) -> String {
    for mut x in bytes {
        x -= 40;
    }
    return String::from_utf8(bytes).unwrap();
}