
use clubs_diamonds::{Searcher, Expression};

fn main() {
    let (count, solutions) =
        Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
            (-1000..=1000).all(|a|
                //expr.apply(&[a]) == if a == 0 {None} else {Some(2^1/a/8-3)}
                //expr.apply(&[a]) == Some(a/24^26-a)
                format!("{expr}").contains("!155")
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

