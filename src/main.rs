use practice_string::daily_activities::generate_daily_activities;

fn main() {
    let activities = [
        "Hiking",
        "Kayaking",
        "Bird Watching",
        "Fishing",
        "Camping",
        "Stargazing",
        "Swimming",
        "Cycling",
    ];

    let start_date = "2025-01-01";
    let end_date = "2025-01-05";

    let daily_activities = generate_daily_activities(&activities, start_date, end_date, 3);

    for (date, activities) in daily_activities {
        println!("{}: {:?}", date, activities);
    }
}