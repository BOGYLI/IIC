
use chrono::prelude::*;

pub fn formatted_time() -> String {
    // Get the current date and time in the Berlin timezone
    let berlin_timezone = FixedOffset::east(1 * 3600); // Berlin is UTC+1
    let berlin_time = Utc::now().with_timezone(&berlin_timezone);

    // Format the date and time as a string
    let formatted_time = berlin_time.format("%d-%m-%Y %H:%M:%S").to_string();
    //let formatted_time = berlin_time.format("%d-%m-%Y %H:%M:%S %:z").to_string();

    formatted_time
}