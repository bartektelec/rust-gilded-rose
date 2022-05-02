use super::*;

proptest! {
    #[test]
    fn backstage_pass_quality_increases_by_1_as_it_ages_long_before_sell_in_date(
            sell_in in 11..=i32::max_value(),
            quality in MIN_QUALITY..MAX_QUALITY) {
        // given
        let expected_quality = quality + 1;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_increases_by_three_near_sell_in_date(
            sell_in in 1..=5,
            quality in MIN_QUALITY..=(MAX_QUALITY - 3)) {
        // given
        let expected_quality = quality + 3;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_increases_by_two_somewhat_near_sell_in_date(
            sell_in in 6..=10,
            quality in MIN_QUALITY..=(MAX_QUALITY - 2)) {
        // given
        let expected_quality = quality + 2;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_does_not_exceed_50_before_sell_in_date(
            sell_in in 1..=i32::max_value(),
            quality in (MAX_QUALITY - 1)..=MAX_QUALITY) {
        // given
        let expected_quality = MAX_QUALITY;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }

    #[test]
    fn backstage_pass_quality_drops_to_0_at_and_after_sell_in_date(
            sell_in in i32::min_value()..=0,
            quality in MIN_QUALITY..=MAX_QUALITY) {
        // given
        let expected_quality = MIN_QUALITY;
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
        let items = vec![Item::new(name, sell_in, quality)];
        let mut rose = GildedRose::new(items);

        // when
        rose.update_quality();

        // then
        assert_eq!(rose.items[0].quality, expected_quality);
    }
    #[test]
    fn inventory_with_multiple_backstage_pass_items_all_age_and_improve_or_degrade(
            item_count in 1_usize..=100) {
        // given
        let mut rng = rand::thread_rng();
        let mut orig_items = Vec::new();
        let mut items = Vec::new();
        let name = String::from("Backstage passes to a TAFKAL80ETC concert");
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
            assert!((rose.items[i].sell_in > 0 &&
                rose.items[i].quality == i32::min(orig_items[i].quality + 1, 50)) ||
                rose.items[i].quality == 0);
        })
     }
}
