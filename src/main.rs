mod utils;
use utils::helper::input_values;
use utils::lib::Item;

mod controllers;
use controllers::crud_operation;

fn main() {
    let (id, name) = input_values();
    let mut item: Item = crud_operation::create_an_item(id, name);
    crud_operation::read_an_item(&mut item);
    crud_operation::update_item(&mut item, 12);
    crud_operation::read_an_item(&mut item);
}
