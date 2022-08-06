use rand::{thread_rng, Rng};

fn main() {
    let mut rng = thread_rng();

// Exclusive range

    let a = "+0023.e+002".parse::<f32>().unwrap();
    let b ="+023".parse::<i32>().unwrap();
    print!("{a} {b}");

}