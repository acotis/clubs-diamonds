
/* NOTE: THIS FILE EXISTS FOR DEVELOPMENT PURPOSES ONLY. It is covered by the CC0
 * license, so feel free to poke around, but don't expect to find anything useful
 * or interesting.
 */

mod self_test_utils;

use clubs_diamonds::{Searcher, Expression};
use clubs_diamonds::Number;

use self_test_utils::judges::*;
use self_test_utils::compare_sets::compare_sets;

use std::io::Write;
use std::io::stdout;
use std::fmt::Debug;

const GREEN: &str = "\x1b[32m";
const RED:   &str = "\x1b[31m";
const BOLD:  &str = "\x1b[1m";
const RESET: &str = "\x1b[0m";

pub fn main() {
    println!();
    all_result_list_tests();
    println!();
}

// Run all tests.

fn all_result_list_tests() {

    // Primes judge.

    println!("i32 primes/7 [7]");

    let (count, results) = 
        Searcher::<i32, 1>::new(i32_primes_7)
            .max_length(7)
            .threads(1)
            .run_silently();

    check_search_count(count, 6094569);
    check_result_set(&results, &mut [
        "2+5*a/3", "2+7*a/4", "2+9*a/5", "2+a*5/3", "2+a*7/4", "2+a*9/5",
        "5*a/3+2", "7*a/4+2", "9*a/5+2", "a*5/3+2", "a*7/4+2", "a*9/5+2",
    ]);

    for result in results {
        check_value(&result, &[0], Some(2));
        check_value(&result, &[1], Some(3));
        check_value(&result, &[2], Some(5));
        check_value(&result, &[3], Some(7));
    }

    // Shift judge.

    println!("usize 113_shift [8]");

    let (count, results) = 
        Searcher::<usize, 1>::new(usize_113_shift)
            .max_length(8)
            .threads(8)
            .run_silently();

    check_search_count(count, 90105278);
    check_result_set(&results, &mut [
        "113>>5%a"
    ]);

    check_value(&results[0], &[0], None);
    check_value(&results[0], &[1], Some(113));
    check_value(&results[0], &[2], Some(56));
    check_value(&results[0], &[3], Some(28));
    check_value(&results[0], &[4], Some(56));
    check_value(&results[0], &[5], Some(113));
    check_value(&results[0], &[6], Some(3));
    check_value(&results[0], &[7], Some(3));
    check_value(&results[0], &[8], Some(3));
    check_value(&results[0], &[9], Some(3));

    // Forty-four judge.
    //
    // For this text, the expected results are simply taken verbatim (with some
    // light manual spot-checking) from the version of clubs which existed on
    // 2025 Oct 15.

    println!("u8 fortyfour [8]");

    let (count, results) =
        Searcher::<u8, 1>::new(u8_fortyfour)
            .max_length(8)
            .threads(8)
            .run_silently();

    check_search_count(count, 90105278);
    check_result_set_from_file(&results, "fortyfour.txt");

    for result in results {
        for a in 0..255 {
            check_value(&result, &[a], Some(a^!0|44));
        }
    }

    // One-variable expressions of length 4-6.

    println!("u64 acceptall [4-6]");

    let (count, results) =
        Searcher::<u64, 1>::new(|_expr| true)
            .min_length(4)
            .max_length(6)
            .threads(16)
            .run_silently();

    check_search_count(count, 472775);
    check_result_set_from_file(&results, "single_variable_4_to_6.txt");

    // One-variable expressions up to length 8.

    println!("u64 acceptall [8]");

    let (count, results) =
        Searcher::<u64, 1>::new(|_expr| true)
            .max_length(8)
            .threads(16)
            .run_silently();

    check_search_count(count, 90105278);
    check_result_set_from_file(&results, "single_variable_8.txt");

    // Two-variable expressions up to length 8.

    println!("u64 acceptall<2> [8]");

    let (count, results) =
        Searcher::<u64, 2>::new(|_expr| true)
            .max_length(8)
            .threads(16)
            .run_silently();

    check_search_count(count, 17132812);
    check_result_set_from_file(&results, "two_variable_8.txt");

    // Three-variable expressions up to length 9.

    println!("u64 acceptall<3> [9]");

    let (count, results) =
        Searcher::<u64, 3>::new(|_expr| true)
            .max_length(9)
            .threads(16)
            .run_silently();

    check_search_count(count, 42738888);
    check_result_set_from_file(&results, "three_variable_9.txt");

    // Check how many single-variable expressions there are of length 9
    // (but not what they are, since there are too many).

    println!("rejectall [9]");

    let (count, results) =
        Searcher::<i64, 1>::new(|_expr| false)
            .max_length(9)
            .threads(16)
            .run_silently();

    check_search_count(count, 1259620267);
    check_result_set(&results, &mut []);

    // One-variable expressions up to length 10 with an arbitrary restriction.

    println!("arbitrary restriction [10]");

    let (count, results) =
        Searcher::<u64, 1>::new(|expr| {
            expr.apply(&[3]) == Some(7) &&
            expr.apply(&[7]) == Some(15) &&
            expr.apply(&[11]) == Some(22)
        })
        .max_length(10)
        .threads(16)
        .run_silently();

    check_search_count(count, 17293380814);
    check_result_set_from_file(&results, "single_variable_10_arbitrary_restriction.txt");
}

// Run one test and print an error if it's wrong.

fn check_result_set<N: Number, const C: usize>(
    results: &[Expression<N, C>],
    expected: &mut [&str]
) {
    print!("  result set...");
    stdout().flush().unwrap();

    match compare_results(expected, results) {
        Ok(()) => {
            println!(" {GREEN}ok{RESET}")
        }
        Err(reason) => {
            println!(" {BOLD}{RED}ERROR{RESET}");

            for line in reason.lines() {
                println!("    {line}");
            }
        }
    }
}

fn check_result_set_from_file<N: Number, const C: usize>(
    results: &[Expression<N, C>],
    expected_file: &str
) {
    check_result_set(
        &results,
        &mut std::fs::read_to_string(&format!("assets/expression_sets/{expected_file}"))
            .expect(&format!("couldn't read expected results file {expected_file}"))
            .lines()
            .collect::<Vec<_>>()
    );
}

// Check that a reported search count matches the expected search count.
// Output a message saying we are doing this and an error if they do not
// match.

fn check_search_count(actual: u128, expected: u128) {
    print!("  search count...");

    if expected == actual {
        println!(" {GREEN}ok{RESET}");
    } else {
        println!(" {BOLD}{RED}ERROR: expected to search {expected} exprs, actually searched {actual}{RESET}");
    }
}

// Silently check that two values are equal, outputting an indented error
// message only if they are different.

fn check_value<N: Number + Debug, const C: usize>(expr: &Expression<N, C>, input: &[N; C], expected: Option<N>) {
    let actual = expr.apply(input);

    if actual != expected {
        println!("  value check... {BOLD}{RED}ERROR: applied {expr} to {input:?}, expected {expected:?}, got {actual:?}{RESET}");
    }
}

// Utility functions.

fn compare_results<N: Number, const C: usize>(expected: &mut [&str], actual: &[Expression<N, C>]) -> Result<(), String> {
    compare_sets_owned(
        expected,
        &actual.iter().map(|x| format!("{x}")).collect::<Vec<_>>()
    )
}

fn compare_sets_owned(expected: &mut [&str], actual: &[String]) -> Result<(), String> {
    compare_sets(
        expected,
        &mut actual.iter().map(|x| x.as_str()).collect::<Vec<_>>()
    )
}

