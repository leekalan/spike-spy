use std::time::Duration;

pub fn format_duration(dur: Duration) -> String {
    let ms = dur.as_millis();
    if ms < 9999 {
        return format!("{ms}ms");
    }

    let secs = dur.as_secs();
    let minutes = secs / 60;
    let seconds = secs % 60;

    if minutes > 0 {
        format!("{minutes}m {seconds}s")
    } else {
        format!("{seconds}s")
    }
}

pub fn format_memory(bytes: u64) -> String {
    if bytes < 9999 {
        format!("{bytes}B")
    } else if (bytes >> 10) < 9999 {
        format!("{}KB", bytes / 1024)
    } else if (bytes >> 20) < 9999 {
        format!("{}MB", bytes / (1024 * 1024))
    } else {
        format!("{}GB", bytes / (1024 * 1024 * 1024))
    }
}
