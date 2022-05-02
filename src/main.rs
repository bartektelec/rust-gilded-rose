#![warn(clippy::all, clippy::nursery, clippy::pedantic, rust_2018_idioms)]
#![forbid(bare_trait_objects)]
#![allow(clippy::match_bool)]
// To use the `unsafe` keyword, change to `#![allow(unsafe_code)]` (do not remove); aids auditing.
#![forbid(unsafe_code)]
// Safety-critical application lints
#![warn(
    clippy::pedantic,
    clippy::float_cmp_const,
    clippy::indexing_slicing,
    clippy::integer_arithmetic,
    clippy::option_unwrap_used,
    clippy::result_unwrap_used
)]
// Uncomment before ship to find redundant crates, debug remnants, missing license files & more
//#![warn(clippy::cargo, clippy::restriction, missing_docs, warnings)]
//#![deny(warnings)]

mod gilded_rose;
use gilded_rose::{GildedRose, Item};

fn main() {
    let items = vec![
        Item::new(String::from("+5 Dexterity Vest"), 10, 20),
        Item::new(String::from("Aged Brie"), 2, 0),
        Item::new(String::from("Elixir of the Mongoose"), 5, 7),
        Item::new(String::from("Sulfuras, Hand of Ragnaros"), 0, 80),
        Item::new(String::from("Sulfuras, Hand of Ragnaros"), -1, 80),
        Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 15, 20),
        Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 10, 49),
        Item::new(String::from("Backstage passes to a TAFKAL80ETC concert"), 5, 49),
        // this conjured item does not work properly yet
        Item::new(String::from("Conjured Mana Cake"), 3, 6),
    ];
    let mut rose = GildedRose::new(items);

    println!("OMGHAI!");
    for i in 0..30 {
        println!("-------- day {} --------", i);
        println!("name, sellIn, quality");
        for item in &rose.items {
            println!("{}, {}, {}", item.name, item.sell_in, item.quality);
        }
        println!();
        rose.update_quality();
    }
}
