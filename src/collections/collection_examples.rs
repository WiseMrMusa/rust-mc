pub fn main() {
    println!("Collection exxamples");

    let array : [usize; 5] = [2; 5];

    let mut vector = vec![5];
    let vec2: Vec<&str> = Vec::new();


    array.to_vec().pop();
    vector.pop();
    vector.push(6);

    let x = vector.get(0).unwrap_or(&6);

    vector.first();

    let name = String::from("Timi");

    let second_char = name.as_bytes().get(90).unwrap_or(&0);

    println!("The second character of timi is {}", second_char);

    println!("The valiue of x is: {}", x);
}