pub fn format_timestamp(timestamp: i64) -> String {
    time::OffsetDateTime::from_unix_timestamp(timestamp).map_or_else(
        |_| timestamp.to_string(),
        |d| {
            format!(
                "{:04}-{:02}-{:02} {:02}:{:02}",
                d.year(),
                d.month() as u8,
                d.day(),
                d.hour(),
                d.minute()
            )
        },
    )
}
