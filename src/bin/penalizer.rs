
use clubs_diamonds::{Searcher, Expression};

fn main() {
    Searcher::<i32, 1>::new(|expr: &Expression::<i32, 1>| {
        expr.apply(&[1]) == Some(2) &&
        expr.apply(&[2]) == Some(3) &&
        expr.apply(&[3]) == Some(5) &&
        expr.apply(&[4]) == Some(7) &&
        expr.apply(&[5]) == Some(11)
    })
    .penalizer(|expr: &Expression::<i32, 1>| {
        if format!("{expr}").chars().filter(|&c| c == 'a').count() > 1 {
            7
        } else {
            0
        }
    })
    .threads(3)
    .run_with_ui();
}

