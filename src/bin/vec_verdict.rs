
use clubs_diamonds::*;

fn main() {
    let solutions =
        Searcher::<i32, 1>::new(move |expr: &Expression::<i32, 1>| {
            (0..=1000).filter(|&i| expr.apply(&[i]) == Some(12345)).collect::<Vec<_>>()
        })
        .inspector(|solution: &Solution<i32, 1, Vec<i32>>| {
            format!(
                "Working inputs: {}",
                solution.data.iter().map(|x| x.to_string()).collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .penalizer(|solution: &Solution<i32, 1, Vec<i32>>| {
            format!("{}", solution.data[0]).len() * 3
        })
        .threads(4)
        .report_every(1<<12)
        .run_with_ui();

    println!("The first three solutions we found were:");
    println!("    — {} ({:?})", solutions[0].expr, solutions[0].data);
    println!("    — {} ({:?})", solutions[1].expr, solutions[1].data);
    println!("    — {} ({:?})", solutions[2].expr, solutions[2].data);

    // solutions is a Vec<Solution::<i32, 1, Vec<i32>>>
}

