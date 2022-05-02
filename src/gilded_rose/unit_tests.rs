#![allow(clippy::indexing_slicing, clippy::integer_arithmetic)]
mod aged_brie;
mod backstage_pass;
mod standard_item;
mod sulfuras;
mod conjured_mana_cake;


use super::{GildedRose, Item};
use proptest::prelude::*;
use rand::Rng;

const MAX_QUALITY: i32 = 50;
const MIN_QUALITY: i32 = 0;