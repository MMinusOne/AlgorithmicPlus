use time::{OffsetDateTime, Date, Month};
use std::error::Error;

pub fn parse_date_string_to_offsettime(date_str: &str) -> Result<OffsetDateTime, Box<dyn Error>> {
    let parts: Vec<&str> = date_str.split('-').collect();
    let year = parts[0].parse::<i32>()?;
    let month = Month::try_from(parts[1].parse::<u8>()?)?;
    let day = parts[2].parse::<u8>()?;
    
    let date = Date::from_calendar_date(year, month, day)?;
    Ok(date.midnight().assume_utc())
}