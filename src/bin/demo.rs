
use clubs_diamonds::{Searcher, ExpressionCore};

fn main() {
    let (count, solutions) =
        Searcher::<usize, 1>::new(|expr: ExpressionCore::<usize, 1>| {
            expr.apply(&[1]) == Some(2) &&
            expr.apply(&[2]) == Some(3) &&
            expr.apply(&[3]) == Some(5) &&
            expr.apply(&[4]) == Some(7) &&
            expr.apply(&[5]) == Some(11)
        })
        .threads(4)
        .run_with_ui();

    println!("Searched {} expressions total", count);
    println!("The first three solutions we found were:");
    println!("    — {}", solutions[0]);
    println!("    — {}", solutions[1]);
    println!("    — {}", solutions[2]);

    // solutions is a Vec<Expression::<usize, 1>>
}

