use std::io;
struct Item {
    id: u32,
    name: String,
}

fn create_an_item(_id: u32, _name: String) -> Item {
    let mut item = Item {
        id: _id,
        name: _name,
    };
    return item;
}

// Make it as whole struct could be printed.
fn read_an_item(item: &mut Item) {
    println!("Item id: {} , name: {}", item.id, item.name);
}

// Make it generic that any value can be taken care here.
fn update_item(_item: &mut Item, _id: u32) {
    _item.id = _id;
}

fn main() {
    println!("Testing!");
    let mut id=0;
    let mut name: String = String::from("Testing");
    let mut item: Item = create_an_item(id, name);
    read_an_item(&mut item);
    update_item(&mut item, 12);
    read_an_item(&mut item);
}
