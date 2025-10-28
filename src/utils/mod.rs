
//!
//! Some value-formatting utility functions used internally by the Clubs UI that are exposed for users who want to format values the same way.
//!
//! Clubs uses chrono for its timestamp needs, so the argument types here are argument types from the chrono crate.
//!

// TODO: It's wrong for a crate to export functions whose signatures contain types from another crate, because then the end user must have the exact same version of the dependency crate installed or else the types will not match up.
//
// One solution is to re-export the types under your own crate. Another is to
// just make the module not public, and I think that's what I prefer.

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

