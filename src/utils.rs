use chrono::Datelike;
use std::collections::HashMap;

use crate::models::SearchResult;

const WEEK_FORMAT: &str = "%m/%d/%Y";
const MONTH_FORMAT: &str = "%m/%Y";
const DAY_FORMAT: &str = "%m/%d/%Y";

fn get_formatted_date(date: chrono::DateTime<chrono::Local>, group_by: &str) -> String {
    match group_by {
        "week" => {
            let days_from_sunday = date.weekday().num_days_from_sunday();
            let start_of_week = date - chrono::Duration::days(days_from_sunday as i64);
            start_of_week.format(WEEK_FORMAT).to_string()
        }
        "month" => date.format(MONTH_FORMAT).to_string(),
        "day" | _ => date.format(DAY_FORMAT).to_string(),
    }
}

pub fn create_map_with_group_by(
    data: Vec<SearchResult>,
    group_by: String,
) -> HashMap<String, Vec<SearchResult>> {
    data.into_iter().fold(HashMap::new(), |mut map, item| {
        let grouped_date = get_formatted_date(item.date, &group_by);
        map.entry(grouped_date).or_insert_with(Vec::new).push(item);
        map
    })
}
