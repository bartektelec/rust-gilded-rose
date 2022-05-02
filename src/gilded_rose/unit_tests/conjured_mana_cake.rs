#![allow(clippy::indexing_slicing)]
use super::*;

proptest! {
    #[test]
    fn conjured_mana_cake_should_decrease_quality_by_2(
            sell_in in 1..=i32::max_value(),
            quality in 2..MAX_QUALITY) {
        // given
        let expected_quality = quality - 2;
        let name = String::from("Conjured Mana Cake");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

}
