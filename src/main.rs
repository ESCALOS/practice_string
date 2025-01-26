use practice_string::daily_activities::{excel::{export_activities_to_excel, load_activities_from_excel}, generate_daily_activities};

fn main() {

    let activities = load_activities_from_excel("activities.xlsx");

    let start_date = "2024-11-01";
    let end_date = "2024-11-30";

    let daily_activities: Vec<(chrono::NaiveDate, Vec<practice_string::daily_activities::model::Activity>)> = generate_daily_activities(activities, start_date, end_date, 3);

    let _ = export_activities_to_excel(&daily_activities);
}