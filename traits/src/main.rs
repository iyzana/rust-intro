fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let a = A { _a: 1 };
    let _b = A { _a: "hi" };

    a.c();
    // b.c(); does not work
}

struct A<T> {
    _a: T,
}

trait B {}

trait C {
    fn c(&self);
}

impl B for i32 {}

impl<T: B> C for A<T> {
    fn c(&self) {}
}
