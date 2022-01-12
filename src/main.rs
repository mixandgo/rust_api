mod to_do;

use to_do::structs::traits::create::Create;
use to_do::to_do_factory;
use to_do::ItemTypes;

fn main() {
    let to_do_item: Result<ItemTypes, &'static str> = to_do_factory("pending", "buy milk");
    match to_do_item.unwrap() {
        ItemTypes::Pending(item) => item.create(&item.super_struct.title),
        ItemTypes::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        ),
    }
}
