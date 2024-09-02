use chrono::{DateTime, Utc};
use leptos::prelude::*;

#[derive(Clone, PartialEq)]
enum CalendarView {
    Day,
    Week,
    Month,
}

#[derive(Clone, PartialEq)]
struct CalendarEvent {
    id: String,
    title: String,
    start: DateTime<Utc>,
    end: DateTime<Utc>,
}
