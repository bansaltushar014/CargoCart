// mod utils;
use crate::utils::lib::Item;
// We are using crate here as it help us to go to root
// We are not using mod here because crud_operation is child of controllers

pub fn create_an_item(_id: u32, _name: String) -> Item {
    let mut item = Item {
        id: _id,
        name: _name,
    };
    return item;
}

// Make it as whole struct could be printed.
pub fn read_an_item(item: &mut Item) {
    println!("Item id: {} , name: {}", item.id, item.name);
}

// Make it generic that any value can be taken care here.
pub fn update_item(_item: &mut Item, _id: u32) {
    _item.id = _id;
}

