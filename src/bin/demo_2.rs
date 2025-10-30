
use clubs_diamonds::{Searcher, Expression};

fn main() {
    Searcher::<i16, 2>::new(|expr: &Expression::<i16, 2>| {
        expr.apply(&[1, 3]) == Some(5) &&
        expr.apply(&[5, -2]) == Some(-6) &&
        expr.apply(&[-8, 7]) == None
    })
    .run_with_ui();
}

