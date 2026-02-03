
use clubs_diamonds::{Searcher, Expression};

fn main() {
    let solutions =
        Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
            (0..100).find(|&i| expr.apply(&[i]) == Some(123456))
        })
        .threads(3)
        .max_len(8)
        .report_every(1<<14)
        .run_with_ui();

    println!("The first three solutions we found were:");
    println!("    — {} ({})", solutions[0].expr, solutions[0].data);
    println!("    — {} ({})", solutions[1].expr, solutions[1].data);
    println!("    — {} ({})", solutions[2].expr, solutions[2].data);

    // solutions is a Vec<Expression::<i32, 1>>
}

