
use clubs_diamonds::*;

fn main() {
    //let mut field = [0_u8; 11];
    //let mut writer = AddSubtractWriter::<i32>::new(0, 11, 0, None);

    //while writer.write(&mut field) {
        //println!("{}", str::from_utf8(&field).unwrap())
    //}

    let mut partition = Partition::new(10);

    loop {
        println!(
            "{}",
            partition.state().iter().map(|&len| "x".repeat(len)).collect::<Vec<_>>().join("|")
        );

        if !partition.next() {
            break
        }
    }

    println!();
    println!("done");
}

