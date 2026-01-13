
use clubs_diamonds::*;
use std::marker::PhantomData;

fn main() {
    let mut expr = Expression::<i32,1> {
        field: vec![0; 6],
        nothing: PhantomData,
    };
    let mut writer = Writer::new(6);

    while writer.write(&mut expr.field) {
        println!("{expr}");
    }

    println!();
    println!("done");
}

