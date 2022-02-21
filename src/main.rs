mod todo;

use todo::{todo_factory, ItemTypes};
use todo::structs::traits::create::Create;

fn main() {
    let todo_item: Result<ItemTypes, &'static str> = todo_factory("pending", "make");
    match todo_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),

        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        ),
    }
}
