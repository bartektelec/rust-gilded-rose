#[cfg(test)]
mod unit_tests;

use std::string;
use std::vec;
use std::cmp;

pub struct Item {
    pub name: string::String,
    pub sell_in: i32,
    pub quality: i32,
}

impl Item {
    pub const fn new(name: String, sell_in: i32, quality: i32) -> Self {
        Self { name, sell_in, quality }
    }
}

pub fn clamp_num(num: i32, min: i32, max: i32) -> i32 {
    cmp::min(max, cmp::max(min, num))
}

pub fn decrease_sell_in(input: i32) -> i32 {
    let mut new_sell_in = input;
    if new_sell_in > i32::min_value() {
        new_sell_in -= 1;
    }

    new_sell_in
}

pub fn map_default_item(item: &Item)-> Item {
    let mut new_quality = item.quality - 1;
    let new_sell_in = decrease_sell_in(item.sell_in);

    if new_sell_in < 0 { new_quality -= 1 }
    
    Item { name: item.name.clone(), sell_in: new_sell_in, quality: clamp_num(new_quality, 0, 50) }
}

pub fn map_aged_brie(item: &Item)-> Item {
    let mut new_quality = item.quality + 1;
    let new_sell_in = decrease_sell_in(item.sell_in);

    if new_sell_in < 0 {new_quality += 1};
    
    Item { name: item.name.clone(), sell_in:  new_sell_in, quality: clamp_num(new_quality, 0, 50) }
}

pub fn map_conjured_cake(item: &Item)-> Item {
    let new_quality = item.quality - 2;
    let new_sell_in = decrease_sell_in(item.sell_in);
    
    Item { name: item.name.clone(), sell_in:  new_sell_in, quality: clamp_num(new_quality, 0, 50) }
}

pub fn map_backstage_pass(item: &Item)-> Item {
    let mut new_quality = item.quality + 1;
    let new_sell_in = decrease_sell_in(item.sell_in);

    if new_sell_in < 10 {new_quality += 1};
    if new_sell_in < 5 {new_quality += 1};
    if new_sell_in < 0 {new_quality = 0}
    
    Item { name: item.name.clone(), sell_in:  new_sell_in, quality: clamp_num(new_quality, 0, 50) }
}
pub fn map_sulfuras(item: &Item)-> Item {
    Item { name: item.name.clone(), sell_in:  item.sell_in, quality: item.quality }
}
 
pub struct GildedRose {
    pub items: vec::Vec<Item>,
}

impl GildedRose {
    pub const fn new(items: vec::Vec<Item>) -> Self {
        Self { items }
    }

    #[allow(clippy::integer_arithmetic)]
    pub fn update_quality(&mut self) {
        self.items = self.items.iter().map(|item| 
            match item.name.as_str() {
                "Conjured Mana Cake" => map_conjured_cake(item),
                 "Aged Brie" => map_aged_brie(item),
                "Backstage passes to a TAFKAL80ETC concert" => map_backstage_pass(item),
                "Sulfuras, Hand of Ragnaros" => map_sulfuras(item),
                _ => map_default_item(item)
        }).collect();

    }
}
