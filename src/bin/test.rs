
use clubs_diamonds::{Searcher, Expression};

fn main() {
    let (count, solutions) =
        Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
            (0..95).all(|a|
                expr.apply(&[a]) == Some(32|5<<a+3)
            )
        })
        .threads(3)
        .run_with_ui();

    println!("Searched {} expressions total", count);
    println!("The first three solutions we found were:");
    println!("    — {}", solutions[0]);
    println!("    — {}", solutions[1]);
    println!("    — {}", solutions[2]);

    // solutions is a Vec<Expression::<i32, 1>>
}

