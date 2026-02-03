
use clubs_diamonds::*;

fn main() {
    let solutions =
        Searcher::<i32, 1>::new(move |expr: &Expression::<i32, 1>| {
            (0..=1000).find(|&i| expr.apply(&[i]) == Some(1_000_000))
        })
        .inspector(|solution: &Solution<i32, 1, i32>| {
            format!("expr({}) works", solution.data)
        })
        .penalizer(|solution: &Solution<i32, 1, i32>| {
            format!("{}", solution.data).len() * 3
        })
        .threads(4)
        .report_every(1<<12)
        .run_with_ui();

    println!("The first three solutions we found were:");
    println!("    — {} ({})", solutions[0].expr, solutions[0].data);
    println!("    — {} ({})", solutions[1].expr, solutions[1].data);
    println!("    — {} ({})", solutions[2].expr, solutions[2].data);

    // solutions is a Vec<Solution::<i32, 1, i32>>
}

