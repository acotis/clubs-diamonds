
use chrono::{DateTime, Local, TimeDelta};

// Format a timestamp like this: "Sat, 2025 May 10 11:40"

pub fn format_timestamp(ts: &DateTime<Local>) -> String {
    ts.format("%a, %Y %b ").to_string()
  +&ts.format("%e").to_string().trim()
  +&ts.format(" %H:%M").to_string()
}

// Format a duration like one of these:
//    — 18s
//    — 4m 6s
//    — 5m 0s
//    — 8h 15m
//    — 3d 4h
//    — 125d 18h

pub fn format_duration(td: &TimeDelta, try_to_exclude_seconds: bool) -> String {
    let days    = td.num_days();
    let hours   = td.num_hours();
    let minutes = td.num_minutes();
    let seconds = td.num_seconds();

    if days > 0 {
        format!("{}d {}h", days, hours - days * 24)
    } else if hours > 0 {
        format!("{}h {}m", hours, minutes - hours * 60)
    } else if minutes > 0 {
        if try_to_exclude_seconds {
            format!("{}m", minutes)
        } else {
            format!("{}m {}s", minutes, seconds - minutes * 60)
        }
    } else {
        format!("{}s", seconds)
    }
}

pub fn with_commas(mut number: u128) -> String {
    if number == 0 {return format!("0");}

    let mut ret = format!("");
    let mut digit_count = 0;

    while number > 0 {
        if digit_count > 0 && digit_count % 3 == 0 {
            ret = format!(",{ret}");
        }

        ret = format!("{}{ret}", (number % 10));
        number /= 10;
        digit_count += 1;
    }

    ret
}

pub fn as_power_of_two(number: u128) -> String {
    format!("2^{:.2}", (number as f64).log2())
}

