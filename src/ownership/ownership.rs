pub fn main() {
    let s = "hello";
    let mut v = String::from("hello");

    v.push_str(", world");

    println!("{}", v);

    let dog = Animal {};

    // dog.bark();

    // Animal::bark(dog);

    // Animal::bark(dog);

    // dog.bark();

    let x = String::from("Your name");

    let y = x.clone();

    // drop(x);

    // println!("{}", x);
    // println!("{}", y);

    let mut collect_ownership = gives_ownership();

    takes_and_gives_back(&mut collect_ownership);

    // takes_and_gives_back.push_str(" with violence");

    // println!("{}", collect_ownership);

    // let referenced_string = String::from("Game on");

    // reference(&referenced_string);

    let s = String::from("Test is ongoing");

    let first_word = &s[0..4];

    println!("{}", first_word);
}

pub fn gives_ownership() -> String {
    String::from("Hello world")
}

pub fn takes_and_gives_back(a: &mut String) {
    a.push_str(", Its time");
}

pub fn reference(a: &String) {
    println!("{}", *a);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// #[derive(Copy)]
pub struct Animal {
    // id: i128,
}

impl Animal {
    pub fn bark(&self) {
        println!("woof woof");
    }

    pub fn moo() {
        println!("moo");
    }
}
