use std::fmt::Display;

pub fn main() {
    println!("Enum examples");

    // let animal = create_animal(String::from("Bingo"), String::from("Elephant")).unwrap_or(Animal::Dog(String::from("Dog")));
    let animal = match create_animal(Animal::Dog(String::from("Bingo"))) {
        Some(val) => val,
        None => Animal::Dog(String::from("Dog")),
    };

    match &animal {
        Animal::Dog(val) => {
            println!("This is {}", val)
        },
        Animal::Cat(_) => todo!(),
        _ => println!("One on the many")
    }

    println!("What is this? This is {}", animal);

    let animal2 = create_animal(Animal::Cat("Garfield".to_string())).unwrap();

    if let Animal::Cat(name) = animal2 {
        println!("This is {}", name);
    }
    

}

#[derive(Debug)]
struct Dog {
    breed: DogBreed
}

#[derive(Debug)]
enum DogBreed {
    Caucasian,
    Boerbull,
    Bingo,
}

impl Dog {
    pub fn bark(&self) {
        println!("Woof woof!!!");
    }
}

impl Display for Animal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}


#[derive(Debug)]
enum Animal {
    Dog(String),
    Cat(String),
    Fish(String),
    Frog(String),
    Goat(String)
}

struct IpAddrV4 {
    addr: Option<u8>,
    addr1: u8,
    dog: Dog
}



pub fn create_animal(animal_type: Animal) -> Option<Animal> {
    // if animal_type == String::from("Dog") {
    //     Some(Animal::Dog(name))
    // } else if animal_type == String::from("Cat") {
    //     Some(Animal::Cat(name))
    // } else if animal_type == String::from("Frog"){
    //     Some(Animal::Frog(name))
    // } else {
    //     None
    // }

    match animal_type {
        Animal::Dog(val) => Some(Animal::Dog(val)),
        Animal::Cat(n) => Some(Animal::Cat(n)),
        Animal::Fish(c) => Some(Animal::Fish(c)),
        Animal::Frog(v) => Some(Animal::Frog(v)),
        Animal::Goat(v) => Some(Animal::Goat(v)),
    }
}



pub mod testing_module_definition {

}