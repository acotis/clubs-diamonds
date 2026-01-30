
use clubs_diamonds::{Searcher, Expression};

fn main() {
    let reference_data = [2, 3, 5, 7, 11];

    let solutions =
        Searcher::<i32, 1>::new(move |expr: &Expression::<i32, 1>| {
            (1..=5).all(|a|
                expr.apply(&[a]) == Some(reference_data[a as usize - 1])
            )
        })
        .threads(3)
        .run_with_ui();

    println!("The first three solutions we found were:");
    println!("    — {}", solutions[0]);
    println!("    — {}", solutions[1]);
    println!("    — {}", solutions[2]);

    // solutions is a Vec<Expression::<i32, 1>>
}

