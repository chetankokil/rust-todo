use rand::seq::SliceRandom;

fn main() {
    let mut rng = rand::thread_rng();
    let mut v: Vec<i32> = (0..10).collect();
    v.shuffle(&mut rng);
    println!("{:?}", v);

    println!("Hello World");

    let test: Vec<i32> = (0..20).collect();
}
