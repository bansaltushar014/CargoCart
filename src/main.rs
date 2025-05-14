use std::io;

struct Item{
    name: String, 
    brand: String, 
    price: u32
}

fn create_item(_name : &str, _brand: &str, _price: &u32, _arrOfItems: &mut Vec<Item>) {
    let item = Item{name: _name.to_string(), brand: _brand.to_string(), price: *_price};
    _arrOfItems.push(item);
}

fn main() {
    let mut arrOfItems:Vec<Item> = vec![];
    let mut input: String = String::new();
    let mut input2: String = String::new();
    let mut input3: String = String::new();
    println!("Enter name:");
    io::stdin().read_line(&mut input).expect("Failed to read");
    let received_name = input.trim(); // Remove trailing newline
    
    println!("Enter brand:");
    io::stdin().read_line(&mut input2).expect("Failed to read");
    let received_brand = input2.trim(); // Remove trailing newline
    
    println!("Enter price:");
    io::stdin().read_line(&mut input3).expect("Failed to read");
    let trimmed = input3.trim(); // Remove trailing newline
    let received_price: u32 = trimmed.parse().expect("Please enter a valid number");
    
    println!("Received: {} {} {}", received_name, received_brand, received_price);

    create_item(&received_name, &received_brand, &received_price, &mut arrOfItems);
    println!("Received: {} {} {}", arrOfItems[0].name, arrOfItems[0].brand, arrOfItems[0].price);
}
