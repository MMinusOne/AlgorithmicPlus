use chrono::{DateTime, NaiveDate, TimeZone, Utc};
use time::{OffsetDateTime, Date, Month};
use std::error::Error;

pub fn parse_date_string_to_offsettime(date_str: &str) -> Result<OffsetDateTime, Box<dyn Error>> {
    let parts: Vec<&str> = date_str.split('-').collect();
    let year = parts[0].parse::<i32>()?;
    let month = Month::try_from(parts[1].parse::<u8>()?)?;
    let day = parts[2].parse::<u8>()?;
    let date = Date::from_calendar_date(year, month, day)?;
    
    return Ok(date.midnight().assume_utc())
}

pub fn parse_date_string_to_utc(date_str: &str) -> Result<DateTime<Utc>, Box<dyn Error>> {
   let naive_date = NaiveDate::parse_from_str(date_str, "%Y-%m-%d")?;
   let naive_datetime = naive_date.and_hms_opt(0, 0, 0).ok_or("Failed to create date time")?;
   
   return Ok(Utc.from_utc_datetime(&naive_datetime)); 
}