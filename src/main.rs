struct Item {
    id: u32,
    name: String,
    // Add other fields as needed
}

fn create_item(id: u32, name: &str) -> Item {
    // Create a new Item
    Item {
        id,
        name: name.to_string(),
    }
}

fn read_item_by_id(items: &[Item], target_id: u32) -> Option<&Item> {
    // Find an item by ID
    items.iter().find(|&item| item.id == target_id)
}

fn update_item_name(items: &mut Vec<Item>, target_id: u32, new_name: &str) -> bool {
    // Update an item's name by ID
    if let Some(item) = items.iter_mut().find(|item| item.id == target_id) {
        item.name = new_name.to_string();
        true
    } else {
        false
    }
}

fn delete_item(items: &mut Vec<Item>, target_id: u32) -> bool {
    // Delete an item by ID
    if let Some(index) = items.iter().position(|item| item.id == target_id) {
        items.remove(index);
        true
    } else {
        false
    }
}

// Functions for CRUD operations will be placed here
fn main() {
    let mut items: Vec<Item> = Vec::new();

    // Creating a new item
    let new_item: Item = create_item(1, "Example");
    items.push(new_item);

    // Displaying the items
    for item in &items {
        println!("ID: {}, Name: {}", item.id, item.name);
    }

    // Reading an item by ID
    let target_id = 1;
    if let Some(item) = read_item_by_id(&items, target_id) {
        println!("Found item with ID {}: {}", target_id, item.name);
    } else {
        println!("Item with ID {} not found", target_id);
    }

    // Updating an item's name
    let target_id = 1;
    let new_name = "Updated Example";
    if update_item_name(&mut items, target_id, new_name) {
        println!("Item with ID {} updated", target_id);
    } else {
        println!("Item with ID {} not found", target_id);
    }

    // Deleting an item
    let target_id = 1;
    if delete_item(&mut items, target_id) {
        println!("Item with ID {} deleted", target_id);
    } else {
        println!("Item with ID {} not found", target_id);
    }
}
