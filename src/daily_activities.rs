use chrono::{NaiveDate, Duration};
use rand::seq::SliceRandom;

pub fn generate_daily_activities<'a>(
    activities: &'a [&'a str],
    start_date: &str,
    end_date: &str,
    per_day: usize,
) -> Vec<(NaiveDate, Vec<&'a str>)> {
    // Parse start and end dates
    let start = NaiveDate::parse_from_str(start_date, "%Y-%m-%d").expect("Invalid start date");
    let end = NaiveDate::parse_from_str(end_date, "%Y-%m-%d").expect("Invalid end date");

    // Calculate the number of days in the range
    let num_days = (end - start).num_days() + 1;

    // Generate activities for each day
    let mut result = Vec::new();
    for day_offset in 0..num_days {
        let current_date = start + Duration::days(day_offset);

        // Shuffle the activities and take up to `per_day` unique ones
        let mut shuffled_activities = activities.to_vec();
        shuffled_activities.shuffle(&mut rand::thread_rng());

        let daily_activities: Vec<&str> = shuffled_activities.into_iter().take(per_day).collect();
        result.push((current_date, daily_activities));
    }

    result
}
