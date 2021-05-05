pub fn say_hello() {
    println!("Hello, world!");
}

pub fn print() {
    let numbers = [1, 2, 3, 4, 5];
    output_sequence(numbers)
}


pub fn print_vec() {
    let numbers = vec![6, 7, 8, 9, 10];
    for n in numbers {
        println!("{}", n);
    }
}


fn output_sequence(numbers: [u8; 5]) {
    for number in numbers.iter() {
        println!("{}", number);
    }
}