use derive_more::Display;
#[derive(serde::Deserialize, Display)]
#[display("Announcements:{:?}", announcements)]
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
pub struct Schedule {
    pub maintenance_windows: Vec<String>,
    pub standard_hours: Vec<StandardHours>,
}
#[derive(serde::Deserialize, Display)]
#[display("Exchange Schedule: {:?}", schedule)]
pub struct GetExchangeScheduleResponse {
    pub schedule: Schedule,
}
#[derive(serde::Deserialize, Display)]
#[display(
    "The Excahnge is Active (T/F) {}, Time to Resume {:?}, Tradiing is Active (T/F) {}",
    exchange_active,
    (exchange_estimated_resume_time),
    trading_active
)]
pub struct GetExcahngeStatus {
    pub exchange_active: bool,
    pub exchange_estimated_resume_time: Option<String>,
    pub trading_active: bool,
}
#[derive(serde::Deserialize, Display)]
#[display("Last time user data was updated: {:?}", as_of_time)]
pub struct GetUserDataTimestampResponse {
    pub as_of_time: String,
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_announcements_deserialization() {
        let json = r#"{"announcements": ["Announcement 1", "Announcement 2"]}"#;
        let response: GetExchangeAnnouncementsResponse = serde_json::from_str(json)
            .unwrap();
        assert_eq!(response.announcements.len(), 2);
        assert_eq!(response.announcements[0], "Announcement 1");
    }
    #[test]
    fn test_announcements_empty() {
        let json = r#"{"announcements": []}"#;
        let response: GetExchangeAnnouncementsResponse = serde_json::from_str(json)
            .unwrap();
        assert_eq!(response.announcements.len(), 0);
    }
    #[test]
    fn test_day_schedule_deserialization() {
        let json = r#"{
            "close_time": "17:00:00",
            "open_time": "09:00:00"
        }"#;
        let schedule: DaySchedule = serde_json::from_str(json).unwrap();
        assert_eq!(schedule.close_time, "17:00:00");
        assert_eq!(schedule.open_time, "09:00:00");
    }
    #[test]
    fn test_standard_hours_deserialization() {
        let json = r#"{
            "start_time": "2025-01-01T00:00:00Z",
            "end_time": "2025-12-31T23:59:59Z",
            "monday": [],
            "tuesday": [],
            "wednesday": [],
            "thursday": [],
            "friday": [],
            "saturday": [],
            "sunday": []
        }"#;
        let hours: StandardHours = serde_json::from_str(json).unwrap();
        assert_eq!(hours.start_time, "2025-01-01T00:00:00Z");
        assert_eq!(hours.monday.len(), 0);
    }
    #[test]
    fn test_schedule_deserialization() {
        let json = r#"{
            "maintenance_windows": ["2025-01-01T00:00:00Z"],
            "standard_hours": []
        }"#;
        let schedule: Schedule = serde_json::from_str(json).unwrap();
        assert_eq!(schedule.maintenance_windows.len(), 1);
        assert_eq!(schedule.standard_hours.len(), 0);
    }
    #[test]
    fn test_exchange_status_active() {
        let json = r#"{
            "exchange_active": true,
            "exchange_estimated_resume_time": null,
            "trading_active": true
        }"#;
        let status: GetExcahngeStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.exchange_active, true);
        assert_eq!(status.trading_active, true);
        assert_eq!(status.exchange_estimated_resume_time, None);
    }
    #[test]
    fn test_exchange_status_inactive() {
        let json = r#"{
            "exchange_active": false,
            "exchange_estimated_resume_time": "2025-01-02T09:00:00Z",
            "trading_active": false
        }"#;
        let status: GetExcahngeStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status.exchange_active, false);
        assert_eq!(
            status.exchange_estimated_resume_time, Some("2025-01-02T09:00:00Z"
            .to_string())
        );
    }
    #[test]
    fn test_user_data_timestamp_deserialization() {
        let json = r#"{"as_of_time": "2025-01-15T12:34:56Z"}"#;
        let response: GetUserDataTimestampResponse = serde_json::from_str(json).unwrap();
        assert_eq!(response.as_of_time, "2025-01-15T12:34:56Z");
    }
}
