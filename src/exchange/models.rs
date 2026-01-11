//! Exchange module models.
//!
//! This module contains data structures for exchange functionality.

use derive_more::Display;


#[derive(serde::Deserialize, Display)]
#[display("Announcements:{:?}", announcements)]


/// Response model for API endpoint.
///
pub struct GetExchangeAnnouncementsResponse {
    pub announcements: Vec<String>,
}


#[derive(serde::Deserialize, Display, Debug)]
#[display("Daily Schedule: closing time{:?}, opening time{:?}", close_time, open_time)]


pub struct DaySchedule {
    pub close_time: String,
    pub open_time: String,
}


#[derive(serde::Deserialize, Display, Debug)]
#[display("Standard Hours: start_time:{:?}, end_time:{:?}", start_time, end_time)]


/// StandardHours data model.
///
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
#[display(
    "Schedule: maintenance_windows:{:?}, standard_hours length:{}",
    maintenance_windows,
    standard_hours.len()
)]


/// Schedule data model.
///
pub struct Schedule {
    pub maintenance_windows: Vec<String>,
    pub standard_hours: Vec<StandardHours>,
}


#[derive(serde::Deserialize, Display)]
#[display("Exchange Schedule: {:?}", schedule)]


/// Response model for API endpoint.
///
pub struct GetExchangeScheduleResponse {
    pub schedule: Schedule,
}


#[derive(serde::Deserialize, Display)]
#[display(
    "The Exchange is Active (T/F) {}, Time to Resume {:?}, Trading is Active (T/F) {}",
    exchange_active,
    (exchange_estimated_resume_time),
    trading_active
)]


/// GetExcahngeStatus data model.
///
pub struct GetExcahngeStatus {
    pub exchange_active: bool,
    pub exchange_estimated_resume_time: Option<String>,
    pub trading_active: bool,
}


#[derive(serde::Deserialize, Display)]
#[display("Last time user data was updated: {:?}", as_of_time)]


/// Response model for API endpoint.
///
pub struct GetUserDataTimestampResponse {
    pub as_of_time: String,
}
