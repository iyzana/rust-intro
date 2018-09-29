mod list;
mod map;
mod string;

fn main() {
    let data = vec![1, 1, 1, 1, 1, 1, 1, 1, 2, 2, 10];

    println!("mean: {}", list::mean(&data));
    println!("median: {}", list::median(&data));
    println!("mode: {:?}", list::mode(&data));

    println!("{}", string::pig_latin("Hi i am your trusted rust program"));

    map::departments();
}
