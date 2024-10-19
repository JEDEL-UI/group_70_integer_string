use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("Type: {}", type_name::<T>());
}

fn main() {
    let number: i32 = 5;
    let string: String = "40".to_string();

    let number_string: String = number.to_string();

    let string_number: i32 = string.parse().unwrap();

    println!("From number to string: {}", number_string);
    print_type_of(&number_string);

    println!("From string to number: {}", string_number);
    print_type_of(&string_number);
}