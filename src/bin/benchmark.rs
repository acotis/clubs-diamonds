
/* NOTE: THIS FILE EXISTS FOR DEVELOPMENT PURPOSES ONLY. It is covered by the CC0
 * license, so feel free to poke around, but don't expect to find anything useful
 * or interesting.
 */

mod self_test_utils;

use std::time::Instant;
use std::io::Write;
use std::fs::File;
use chrono::{DateTime, Local};

use clubs_diamonds::Searcher;
use clubs_diamonds::Number;

use self_test_utils::outcome_colors::{apply_color_spectrum, ansi_truecolor};
use self_test_utils::stats::{mean_stddev, two_sided_welch_t_test};
use self_test_utils::judges::*;

const REF_FILE: Option<&str> = Some("assets/benchmarks/reference_benchmark.txt");

// Input type: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize
// Number of inputs: 1, 2, 3, 4, more?
// Judge: reject-all, primes 7/11/13/17/19/more?, multi-apply 2/3/4/more?, SBLT
// Min length: 0, 8, 12, 14, 16, 18, 20
// Maximum length to search: 3 4 5 6 7 8 9 10 11? 12? 13? 14?
// Number of threads: 1 2 4 8 12 16
// Max constant: None/0/1/2/3/9/99/999/9999/All
// Negation allowed: yes/no

pub fn main() {
    println!();
    
    let mut file = File::create(format!("assets/benchmarks/{}", format_timestamp(&Local::now()))).unwrap();

    for task in [
        // Short benchmarks.
        || bench("   u8 primes/7  [8]          ", 10, Searcher::<   u8, 1>::new(   u8_primes_7 ).threads( 1).max_length( 8), (90105278, 58)),
        || bench("   i8 primes/7  [8]          ", 10, Searcher::<   i8, 1>::new(   i8_primes_7 ).threads( 1).max_length( 8), (90105278, 76)),
        || bench("  u16 primes/7  [8]          ", 10, Searcher::<  u16, 1>::new(  u16_primes_7 ).threads( 1).max_length( 8), (90105278, 58)),
        || bench("  i16 primes/7  [8]          ", 10, Searcher::<  i16, 1>::new(  i16_primes_7 ).threads( 1).max_length( 8), (90105278, 76)),
        || bench("  u32 primes/7  [8]          ", 10, Searcher::<  u32, 1>::new(  u32_primes_7 ).threads( 1).max_length( 8), (90105278, 58)),
        || bench("  i32 primes/7  [8]          ", 10, Searcher::<  i32, 1>::new(  i32_primes_7 ).threads( 1).max_length( 8), (90105278, 76)),
        || bench("  u64 primes/7  [8]          ", 10, Searcher::<  u64, 1>::new(  u64_primes_7 ).threads( 1).max_length( 8), (90105278, 58)),
        || bench("  i64 primes/7  [8]          ", 10, Searcher::<  i64, 1>::new(  i64_primes_7 ).threads( 1).max_length( 8), (90105278, 76)),
        || bench(" u128 primes/7  [8]          ", 10, Searcher::< u128, 1>::new( u128_primes_7 ).threads( 1).max_length( 8), (90105278, 58)),
        || bench(" i128 primes/7  [8]          ", 10, Searcher::< i128, 1>::new( i128_primes_7 ).threads( 1).max_length( 8), (90105278, 76)),
        || bench("usize primes/7  [8]          ", 10, Searcher::<usize, 1>::new(usize_primes_7 ).threads( 1).max_length( 8), (90105278, 58)),
        || bench("isize primes/7  [8]          ", 10, Searcher::<isize, 1>::new(isize_primes_7 ).threads( 1).max_length( 8), (90105278, 76)),

        // Compare input type.
        || bench("   u8 primes/7  [9]          ", 10, Searcher::<   u8, 1>::new(   u8_primes_7 ).threads( 1).max_length( 9), (1259620267, 2854)),
        || bench("   i8 primes/7  [9]          ", 10, Searcher::<   i8, 1>::new(   i8_primes_7 ).threads( 1).max_length( 9), (1259620267, 1992)),
        || bench("  u16 primes/7  [9]          ", 10, Searcher::<  u16, 1>::new(  u16_primes_7 ).threads( 1).max_length( 9), (1259620267, 3324)),
        || bench("  i16 primes/7  [9]          ", 10, Searcher::<  i16, 1>::new(  i16_primes_7 ).threads( 1).max_length( 9), (1259620267, 3548)),
        || bench("  u32 primes/7  [9]          ", 10, Searcher::<  u32, 1>::new(  u32_primes_7 ).threads( 1).max_length( 9), (1259620267, 3312)),
        || bench("  i32 primes/7  [9]          ", 10, Searcher::<  i32, 1>::new(  i32_primes_7 ).threads( 1).max_length( 9), (1259620267, 3536)),
        || bench("  u64 primes/7  [9]          ", 10, Searcher::<  u64, 1>::new(  u64_primes_7 ).threads( 1).max_length( 9), (1259620267, 3302)),
        || bench("  i64 primes/7  [9]          ", 10, Searcher::<  i64, 1>::new(  i64_primes_7 ).threads( 1).max_length( 9), (1259620267, 3528)),
        || bench(" u128 primes/7  [9]          ", 10, Searcher::< u128, 1>::new( u128_primes_7 ).threads( 1).max_length( 9), (1259620267, 3304)),
        || bench(" i128 primes/7  [9]          ", 10, Searcher::< i128, 1>::new( i128_primes_7 ).threads( 1).max_length( 9), (1259620267, 3524)),
        || bench("usize primes/7  [9]          ", 10, Searcher::<usize, 1>::new(usize_primes_7 ).threads( 1).max_length( 9), (1259620267, 3302)),
        || bench("isize primes/7  [9]          ", 10, Searcher::<isize, 1>::new(isize_primes_7 ).threads( 1).max_length( 9), (1259620267, 3528)),

        // Compare task.
        || bench("  u64 rejectall [9]      4T  ", 10, Searcher::<  u64, 1>::new(  u64_rejectall).threads( 4).max_length( 9), (1259620267, 0)),
        || bench("  u64 primes/7  [9]      4T  ", 10, Searcher::<  u64, 1>::new(  u64_primes_7 ).threads( 4).max_length( 9), (1259620267, 3302)),
        || bench("  u64 primes/11 [9]      4T  ", 10, Searcher::<  u64, 1>::new(  u64_primes_11).threads( 4).max_length( 9), (1259620267, 0)),

        // Try two-input expressions.
        || bench("   i8 qubee     [10]         ", 10, Searcher::<   i8, 2>::new(   i8_qubee    ).threads( 1).max_length(10), (4161267464, 1)),
        || bench("  u32 qubee     [10]         ", 10, Searcher::<  u32, 2>::new(  u32_qubee    ).threads( 1).max_length(10), (4161267464, 3)),
        || bench("isize qubee     [10]         ", 10, Searcher::<isize, 2>::new(isize_qubee    ).threads( 1).max_length(10), (4161267464, 1)),

        // Compare thread count.
        || bench("   u8 primes/11 [8]          ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads( 1).max_length( 8), (90105278, 0)),
        || bench("   u8 primes/11 [8]      4T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads( 4).max_length( 8), (90105278, 0)),
        || bench("   u8 primes/11 [8]     12T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads(12).max_length( 8), (90105278, 0)),
        || bench("   u8 primes/11 [8]     16T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads(16).max_length( 8), (90105278, 0)),
        || bench("   u8 primes/11 [9]          ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads( 1).max_length( 9), (1259620267, 0)),
        || bench("   u8 primes/11 [9]      4T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads( 4).max_length( 9), (1259620267, 0)),
        || bench("   u8 primes/11 [9]     12T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads(12).max_length( 9), (1259620267, 0)),
        || bench("   u8 primes/11 [9]     16T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads(16).max_length( 9), (1259620267, 0)),
        || bench("   u8 primes/11 [10]         ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads( 1).max_length(10), (17293380814, 21)),
        || bench("   u8 primes/11 [10]     4T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads( 4).max_length(10), (17293380814, 21)),
        || bench("   u8 primes/11 [10]    12T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads(12).max_length(10), (17293380814, 21)),
        || bench("   u8 primes/11 [10]    16T  ", 10, Searcher::<   u8, 1>::new(   u8_primes_11).threads(16).max_length(10), (17293380814, 21)),
    ] {
        let (title, data) = task();
        let line = format!("{title}, {}\n", data.iter().map(|&d| d.to_string()).collect::<Vec<_>>().join(" "));
        file.write_all(line.as_bytes()).unwrap();
        file.flush().unwrap();
    }

    println!();
}

fn bench<N: Number, const C: usize>(
    description: &str,
    reps: usize,
    searcher: Searcher<N, C>,
    (expected_search_count, expected_result_count): (u128, usize),
)
    -> (&str, Vec<f64>)
{

    // Formatting constants.

    const BOLD:     &'static str = "\x1b[1m";
    const RED:      &'static str = "\x1b[31m";
    const REF:      &'static str = "\x1b[38;5;244m";
    const REAL:     &'static str = "\x1b[38;5;253m";
    const DESC:     &'static str = "";
    const RESET:    &'static str = "\x1b[0m";

    const WIDTH:        usize = 86;
    const TITLE_WIDTH:  usize = 29;
    const GAP_WIDTH:    usize = 2;
    const MARGIN_WIDTH: usize = 2;
    const MEAN_WIDTH:   usize = 6;
    const STDDEV_WIDTH: usize = 4;
    const BAR_WIDTH:    usize = WIDTH - TITLE_WIDTH - GAP_WIDTH - MEAN_WIDTH - STDDEV_WIDTH - 8 - MARGIN_WIDTH;

    // Get the reference data, if there is any.

    let reference_dataset = REF_FILE.and_then(|file| {
        let string = std::fs::read_to_string(file).ok()?;

        for line in string.lines() {
            let units = line.split(",").collect::<Vec<_>>();

            if units[0] == description {
                return Some(units[1].split(" ").flat_map(str::parse::<f64>).collect::<Vec<_>>());
            }
        }

        None
    });
    
    // Print the header.

    print!("{DESC}{description:TITLE_WIDTH$}{RESET}{}", " ".repeat(GAP_WIDTH));

    if let Some(ref reference_dataset) = reference_dataset {
        let (reference_mean, reference_stddev) = mean_stddev(&reference_dataset);
        print!("{REF}{reference_mean:MEAN_WIDTH$.2}s ± {reference_stddev:STDDEV_WIDTH$.2}{RESET} ");
    } else {
        print!("{REF}{}{RESET} ", "—".repeat(MEAN_WIDTH + 4 + STDDEV_WIDTH))
    }

    std::io::stdout().flush().unwrap();

    // Do the runs.

    let mut times = vec![];

    for trial in 1..=reps {
        let pos = ((trial - 1) as f64 / reps as f64 * (BAR_WIDTH - 3) as f64) as usize;
        let bar = format!("[{}>{}]", "=".repeat(pos), " ".repeat(BAR_WIDTH - 3 - pos));

        print!("{bar}\x1b[{}D", BAR_WIDTH);
        std::io::stdout().flush().unwrap();

        let before = Instant::now();
        let (total, exprs) = searcher.run_silently();
        let after = Instant::now();
        let seconds = (after - before).as_micros() as f64 / 1000000.0;
    
        times.push(seconds);

        // Check error conditions that can disqualify a trial.

        let error = if total != expected_search_count {
            Some(format!("Trial #{trial} had incorrect total search count {}", total))
        } else if exprs.len() != expected_result_count {
            Some(format!("Trial #{trial} had incorrect solution count {}", exprs.len()))
        } else {
            None
        };

        if let Some(error) = error {
            println!("{RED}{BOLD}{error:BAR_WIDTH$}{RESET}");
            return (description, vec![]);
        }
    }

    // Compute the "local" stats.

    let (mean, stddev) = mean_stddev(&times);

    // Compute a pretty and informative color for the observed standard
    // deviation (large deviations should be marked as red so that we know
    // something is weird).

    let ratio = stddev / mean;
    let stddev_color = apply_color_spectrum(ratio, &[
        (0.10, (0.7, 0.7, 0.7)),
        (0.50, (1.0, 0.2, 0.2)),
    ]);

    let stddev_atc = ansi_truecolor(stddev_color);

    // Print the local stats.

    print!("{REAL}{mean:MEAN_WIDTH$.2}s {stddev_atc}± {stddev:STDDEV_WIDTH$.2}{RESET}    ");

    // If we are comparing to an existing dataset, print the comparison stats.

    if let Some(ref reference_dataset) = reference_dataset {
        let (reference_mean, reference_stddev) = mean_stddev(&reference_dataset);
        let stats = two_sided_welch_t_test(&reference_dataset, &times);

        let percent = (mean - reference_mean) / reference_mean * 100.0;
        let z = (mean - reference_mean) / reference_stddev;
        let p = stats.p_value;

        // Compute a pretty and informative color for the outcome (good outcomes
        // should be green and bad outcomes should be yellow or red so we can tell
        // at a glance how good the run was).

        let result_color = if z < 0.0 {
            apply_color_spectrum(p, &[
                (0.0, (0.0, 0.9, 0.0)),
                (0.2, (0.9, 0.9, 0.9)),
            ])
        } else {
            apply_color_spectrum(p, &[
                (0.0, (0.9, 0.0, 0.0)),
                (0.2, (0.9, 0.9, 0.0)),
                (0.4, (0.9, 0.9, 0.9)),
            ])
        };

        let result_atc = ansi_truecolor(result_color);

        print!("{result_atc}{percent:+.0}%, p = {p:.2}{RESET}");
    }

    // Clear the rest of the line and go to the next line.

    println!("\x1b[K");

    // Return the times.

    (description, times)
}

// Format a timestamp like this: "Sat, 2025 May 10 11:40"

pub fn format_timestamp(ts: &DateTime<Local>) -> String {
    ts.format("%a, %Y %b ").to_string()
  +&ts.format("%e").to_string().trim()
  +&ts.format(" %H:%M").to_string()
}

