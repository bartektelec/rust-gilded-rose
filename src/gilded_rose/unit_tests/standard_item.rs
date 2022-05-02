use super::*;

proptest! {
    #[test]
    fn arbitrary_names_dont_crash(name in "\\PC*") {
        // given
        let sell_in = 0;
        let quality = MIN_QUALITY;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        // no panic occurred
    }

    #[test]
    fn item_quality_does_not_go_below_0(
            sell_in in i32::min_value()..=i32::max_value(),
            quality in 0_i32..=MIN_QUALITY + 1) {
        // given
        let expected_quality = 0;
        let name = String::from("foo");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn item_sell_in_goes_below_0(
            sell_in in i32::min_value()..=0) {
        // given
        let expected_sell_in = sell_in.saturating_sub(1);
        let name = String::from("foo");
        let quality = MIN_QUALITY;
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].sell_in, expected_sell_in);
    }

    #[test]
    fn non_zero_item_quality_decreases_by_1_before_sell_in_date(
            sell_in in 1..=i32::max_value(),
            quality in 1..=MAX_QUALITY) {
        // given
        let expected_quality = quality - 1;
        let name = String::from("foo");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn non_zero_item_quality_decreases_by_2_at_and_after_sell_in_date(
            sell_in in i32::min_value()..=0,
            quality in 1..=MAX_QUALITY) {
        // given
        let expected_quality = i32::max(quality - 2, MIN_QUALITY);
        let name = String::from("foo");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn inventory_with_multiple_standard_items_all_age_and_degrade(
            item_count in 1_usize..=100) {
        // given
        let mut rng = rand::thread_rng();
        let mut orig_items = Vec::new();
        let mut items = Vec::new();
        let name = String::from("foo");
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
            assert!(rose.items[i].quality < orig_items[i].quality ||
                    rose.items[i].quality == MIN_QUALITY);
        })
     }
}

#[test]
pub fn item_sell_in_saturates_at_i32_min_value() {
    // given
    let expected_sell_in = i32::min_value();
    let name = String::from("foo");
    let sell_in = i32::min_value();
    let quality = MIN_QUALITY;
    let items = vec![Item::new(name, sell_in, quality)];
    let mut rose = GildedRose::new(items);

    // when
    rose.update_quality();

    // then
    assert_eq!(rose.items[0].sell_in, expected_sell_in);
}
