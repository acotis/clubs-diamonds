
use clubs_diamonds::*;
use std::marker::PhantomData;

fn main() {
    let mut expr = Expression::<i32,1> {
        field: vec![0; 6],
        nothing: PhantomData,
    };

    let mut writer = Writer::new(6, WriterContext {location: Location::TOP, const_allowed: true});
    let mut count = 0;

    while writer.write(&mut expr.field) {
        count += 1;
        println!("{count:6}.  {expr}");
    }

    println!();
    println!("done");
}

