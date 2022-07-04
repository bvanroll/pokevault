pub struct Save {
    pub(crate) trainer_name: String,
    pub(crate) playtime_hours: u8 //in seconds, biggest number should be like 4.3 mill, biggest available time (999:95) is like... 3.6 mill, maybe a little more. 3.7 max
}