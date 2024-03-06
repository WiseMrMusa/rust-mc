use std::io;

pub fn return_returned_value() -> isize {
    let returned_val = get_number();
    returned_val
}

pub fn get_number() -> isize {
    6
}

/// This is the main function
/// nunoinoinojnok
pub fn array_example() {
    let mut arr = [1, 2, 3, 4, 5];

    println!("Please specify the index of the element you want to access");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let ind: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[ind];

    println!("The element at index {} is {}", ind, element);

    let number = get_number();
}

// pub fn main() {
//     let x = 5;

// for i in 0..=10 {
//     if  i == 2 {
//         println!("Here we go: {i}");
//     } else if i == 3 {
//         println!("3");
//     } else if i == 4 {
//         println!("4");
//     } else {
//         println!("Not there yet");
//     }
// }

// let mut i = 0;
// loop {
//     if i == 5 {
//         println!("Done");
//         break;
//     }
//     println!("{i}");
//     i += 1;
// }
// }

pub fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
