use derive_more::Display;
use serde::{Deserialize, Serialize};

#[derive(serde::Deserialize, Display)]
#[display("Announcements:{:?}", announcements)]
pub struct GetExchangeAnnouncementsResponse {
    pub announcements: Vec<String>
} 

#[derive(serde::Deserialize, Display, Debug)]
#[display("Daily Schedule: closing time{:?}, opening time{:?}", close_time, open_time)]
pub struct DaySchedule {
    pub close_time: String,
    pub open_time: String
}

#[derive(serde::Deserialize, Display, Debug)]
#[display("Standard Hours: start_time:{:?}, end_time:{:?}", start_time, end_time)]
pub struct StandardHours {
    pub start_time: String,
    pub end_time: String,
    pub monday: Vec<DaySchedule>,
    pub tuesday: Vec<DaySchedule>,
    pub wednesday: Vec<DaySchedule>,
    pub thursday: Vec<DaySchedule>,
    pub friday: Vec<DaySchedule>,
    pub saturday: Vec<DaySchedule>,
    pub sunday: Vec<DaySchedule>,
}

#[derive(serde::Deserialize, Display, Debug)]
#[display("Schedule: maintenance_windows:{:?}, standard_hours length:{}", maintenance_windows, standard_hours.len())]
pub struct Schedule {
    pub maintenance_windows: Vec<String>,
    pub standard_hours: Vec<StandardHours>,
}

#[derive(serde::Deserialize, Display)]
#[display("Exchange Schedule: {:?}", schedule)]
pub struct GetExchangeScheduleResponse {
    pub schedule: Schedule,
}
