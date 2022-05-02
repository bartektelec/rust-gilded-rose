#![allow(clippy::indexing_slicing)]
use super::*;

proptest! {
    #[test]
    fn aged_brie_quality_increases_by_1_as_it_ages_before_sell_in_date(
            sell_in in 1..=i32::max_value(),
            quality in 0..MAX_QUALITY) {
        // given
        let expected_quality = quality + 1;
        let name = String::from("Aged Brie");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn aged_brie_quality_increases_by_two_at_and_after_sell_in_date(
        sell_in in i32::min_value()..=0,
        quality in 0..=(MAX_QUALITY - 2)) {
        // given
        let expected_quality = quality + 2;
        let name = String::from("Aged Brie");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn aged_brie_quality_does_not_exceed_50(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in (MAX_QUALITY - 1)..=MAX_QUALITY) {
        // given
        let expected_quality = MAX_QUALITY;
        let name = String::from("Aged Brie");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn inventory_with_multiple_aged_brie_items_all_age_and_improve(
            item_count in 1_usize..=100) {
        // given
        let mut rng = rand::thread_rng();
        let mut orig_items = Vec::new();
        let mut items = Vec::new();
        let name = String::from("Aged Brie");
        (0..item_count).for_each(|_| {
            let sell_in = rng.gen();
            let quality = rng.gen_range(MIN_QUALITY, MAX_QUALITY + 1);
            orig_items.push(Item::new(name.clone(), sell_in, quality));
            // cannot alter `Items` according to requirements,
            // so create duplicate collection instead of adding `Clone` trait
            items.push(Item::new(name.clone(), sell_in, quality));
        });
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        (0..item_count).for_each(|i| {
            assert_eq!(rose.items[i].name, orig_items[i].name);
            assert_eq!(rose.items[i].sell_in, orig_items[i].sell_in.saturating_sub(1));
            assert!(rose.items[i].quality > orig_items[i].quality || rose.items[i].quality ==
            MAX_QUALITY);
        })
     }
}
