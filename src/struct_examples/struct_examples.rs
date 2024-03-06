pub fn main() {
    let tup = (5_usize, "Game", [1, 2, 3]);

    let (age, name, game_id) = tup;

    let skin = "fur";

    let dog = Animal {
        specie: String::from("dog"),
        no_of_legs: 4,
        skin,
        is_mamal: true,
        habitat: "land".to_string(),
    };

    let dog1 = Dog;
    let dog2 = Dog.walk();
    let dog3 = dog2.skin;

    drop(skin);

    let monkey = Animal {
        no_of_legs: 6,
        ..dog.clone()
    };

    println!("A monkey is a mamal: {}", monkey.is_mamal);
    
    let mut cat = Animal::create_animal("Cat".to_string(), Some(4), "fur", "land".to_string(), true);
    
    println!("A cat has {} number of legs", cat.get_number_of_legs());
    
    cat.update_num_of_legs(25);
    
    println!("A cat has {} number of legs", cat.get_number_of_legs());

    println!("A {} converted to {} ", dog.clone().skin, dog.convert_to_cat().specie);


    let is_equal = dog == cat;

    let num = 5;

    let color_code = RGB(0, 0, 255);

    color_code.2;
}

#[derive(Debug, Clone, PartialEq)]
pub struct Animal<'a> {
    specie: String,
    no_of_legs: u8,
    skin: &'a str,
    habitat: String,
    is_mamal: bool,
}

pub trait  AnimalTrait {
    fn walk(&self) -> Animal;
    fn talk();
}


#[derive(Debug, Clone)]
pub struct Dog;
pub struct Cat;

impl Cat {

}

impl AnimalTrait for Dog {
    fn walk(&self) -> Animal {
        Animal { specie: String::from("value"), no_of_legs: 4, skin:"brrr", habitat: String::from(""), is_mamal: true }
    }

    fn talk() {
        todo!()
    }
}

impl AnimalTrait for Cat {
    fn walk(&self) -> Animal {
        todo!()
    }

    fn talk() {
        todo!()
    }
}

impl <'a> Animal<'a> {
    pub fn create_animal(specie: String, no_of_legs: Option<u8>, skin: &str, habitat: String, is_mamal: bool) -> Animal {
        
        let nol = match  no_of_legs {
            Some(val) => val,
            None => 0,
        };
        Animal {
            specie,
            no_of_legs: nol,
            skin,
            habitat,
            is_mamal,
        }
    }

    pub fn get_number_of_legs(&self) -> u8 {
        self.no_of_legs
    }

    pub fn update_num_of_legs(&mut self, new_num_of_legs: u8) {
        self.no_of_legs = new_num_of_legs;
    }

    pub fn convert_to_cat(&self) -> Animal<'a> {
        Self {
            specie: String::from("Cat"),
            ..self.clone()
        }
    }

    pub fn update_skin(&mut self, new_skin: &'a str) {
        self.skin = new_skin;
    }

}

#[derive(Debug, Clone, PartialEq)]
pub struct RGB(u8, u8, u8);