
use clubs_diamonds::*;
use std::marker::PhantomData;

fn main() {
    let mut expr = Expression::<i32,1> {
        field: vec![0; 11],
        nothing: PhantomData,
    };
    let mut writer = OrWriter::new(11);

    while writer.write(&mut expr.field) {
        println!("{expr}");
    }

    println!();
    println!("done");
}

