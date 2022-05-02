use super::*;

proptest! {
    #[test]
    fn sulfuras_sell_in_does_not_decrease_in_quality(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in MIN_QUALITY..=MAX_QUALITY) {
        // given
        let expected_quality = quality;
        let name = String::from("Sulfuras, Hand of Ragnaros");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn sulfuras_sell_in_does_not_age(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in MIN_QUALITY..=MAX_QUALITY) {
        // given
        let expected_sell_in = sell_in;
        let name = String::from("Sulfuras, Hand of Ragnaros");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].sell_in, expected_sell_in);
    }

    #[test]
    fn inventory_with_multiple_sulfuras_items_all_age_and_improve(
            item_count in 1_usize..=100) {
        // given
        let mut rng = rand::thread_rng();
        let mut orig_items = Vec::new();
        let mut items = Vec::new();
        let name = String::from("Sulfuras, Hand of Ragnaros");
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
            assert_eq!(rose.items[i].sell_in, orig_items[i].sell_in);
            assert_eq!(rose.items[i].quality, orig_items[i].quality);
        })
     }
}
