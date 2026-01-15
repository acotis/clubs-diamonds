
use clubs_diamonds::*;
use std::marker::PhantomData;

fn main() {
    let len = 9;

    let mut expr = Expression::<i32,1> {
        field: vec![0; len],
        nothing: PhantomData,
    };

    let mut writer = Writer::new(len, WriterContext {location: Location::TOP, const_allowed: true});
    let mut count = 0;

    while writer.write(&mut expr.field) {
        count += 1;
        println!("{count:6}.  {expr}");
    }

    println!();
    println!("done");
}

