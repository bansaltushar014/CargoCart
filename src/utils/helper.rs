use std::io;

pub fn input_values() -> (u32, String) {
    let mut index = String::new();
    println!("Enter the Id: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let _id: u32 = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let mut index = String::new();
    println!("Enter the Name: ");
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let _name: String = index.trim().to_string();

    return (_id, _name);
}
