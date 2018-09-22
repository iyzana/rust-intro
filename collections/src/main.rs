mod list;
mod string;
mod map;

fn main() {
    let a: String = string::pig_latin("Hi i am your trusted rust program");
    println!("{}", a.as_str());

    map::departments();
}
