
use clubs_diamonds::{Searcher, Expression};

fn main() {
    let solutions =
        Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
            expr.apply(&[1]) == Some(2) &&
            expr.apply(&[2]) == Some(3) &&
            expr.apply(&[3]) == Some(5) &&
            expr.apply(&[4]) == Some(7) &&
            expr.apply(&[5]) == Some(11)
        })
        .inspector(|expr| {
            format!("long long long long long long long long long long long long long long long long long long")
        })
        .threads(3)
        .run_with_ui();

    println!("The first three solutions we found were:");
    println!("    — {}", solutions[0]);
    println!("    — {}", solutions[1]);
    println!("    — {}", solutions[2]);

    // solutions is a Vec<Expression::<i32, 1>>
}

