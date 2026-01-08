
use clubs_diamonds::*;

fn main() {
    let mut field = [0_u8; 10];
    let mut writer = Writer::<i32>::new(0, 10, 0, None);

    while writer.write(&mut field) {
        println!("{}", str::from_utf8(&field).unwrap())
    }

    println!();
    println!("done");
}

