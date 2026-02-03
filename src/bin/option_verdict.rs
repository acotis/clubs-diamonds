
use clubs_diamonds::{Searcher, Expression};

fn main() {
    let solutions =
        Searcher::<i64, 1>::new(move |expr: &Expression::<i64, 1>| {
            (0..=1000).find(|&i| expr.apply(&[i]) == Some(1_000_000))
        })
        .inspector(|solution| {
            format!("expr({}) works", solution.data)
        })
        .penalizer(|solution| {
            format!("{}", solution.data).len() * 3
        })
        .threads(4)
        .report_every(1<<12)
        .run_with_ui();

    println!("The first three solutions we found were:");
    println!("    — {} ({})", solutions[0].expr, solutions[0].data);
    println!("    — {} ({})", solutions[1].expr, solutions[1].data);
    println!("    — {} ({})", solutions[2].expr, solutions[2].data);

    // solutions is a Vec<Expression::<i32, 1>>
}

