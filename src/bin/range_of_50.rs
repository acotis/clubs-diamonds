
use clubs_diamonds::{Searcher, Expression};

fn main() {
    let solutions =
        Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {

            let domain = &[
                0, 1, 2, 3, 4,
                5, 6, 7, 8, 9,
                10, 11, 12, 13, 14,
                15, 16, 17, 18, 19,
                20, 21, 22, 23, 24,
                !25, !26, !27, !28, !29,
                !30, !31, !32, !33, !34,
                !35, !36, !37, !38, !39,
                !40, !41, !42, !43, !44,
                !45, !46, !47, !48, !49,
            ];

            let range = &[
                1, -1, 0, 0, 7,
                1, -8, 0, 0, 7,
                1, -1, 9, 0, 7,
                1, -1, 0, 16, 7,
                1, -1, 0, 40, 7,
                1, -8, 0, 0, 7,
                1, -1, 0, 16, 7,
                1, -1, 0, 40, 7,
                1, -8, 0, 0, 7,
                1, -1, 0, 40, 7,
            ];

            (0..50).all(|i|
                expr.apply(&[domain[i]]) == Some(range[i])
            )
        })
        .threads(1)
        .run_with_ui();

    println!("The first three solutions we found were:");
    println!("    — {}", solutions[0]);
    println!("    — {}", solutions[1]);
    println!("    — {}", solutions[2]);

    // solutions is a Vec<Expression::<i32, 1>>
}

