use crate::item::Item;
use crate::item::ItemType;
use rakelog::rakeInfo;
use rand::Rng;
use rand::{self, seq::IndexedRandom};

// This system works via a dice roll so the roll can be in the range of 0-100.
// this gives the Ultra Rare items a ~2% chance of being pulled and teh Rare items a %5 chance
const ULTRA_RARE_ITEM_VALUE: u32 = 98;
const RARE_ITEM_VALUE: u32 = 96;

#[derive(Debug, Clone)]
pub struct Shop {
    pub common_items: Vec<Item>,
    pub rare_items: Vec<Item>,
    pub ultra_rare_items: Vec<Item>,
}

impl Shop {
    pub fn new() -> Self {
        let common_items = Shop::common_tiems();
        let rare_items = Shop::rare_items();
        let ultra_rare_items = Shop::ultra_rare_items();
        Self {
            common_items,
            rare_items,
            ultra_rare_items,
        }
    }

    pub fn get_shop_item(&mut self) -> Option<&Item> {
        let mut rng = rand::rng();
        let dice_roll = rng.random_range(0..100);

        rakeInfo!("Shop item: {dice_roll}");

        if dice_roll >= ULTRA_RARE_ITEM_VALUE {
            rakeInfo!("Got ULTRA RARE ITEM");
            return self.ultra_rare_items.choose(&mut rng);
        } else if dice_roll > RARE_ITEM_VALUE && dice_roll < ULTRA_RARE_ITEM_VALUE {
            rakeInfo!("Got RARE ITEM");
            return self.rare_items.choose(&mut rng);
        } else {
            rakeInfo!("Got Common ITEM");
            return self.common_items.choose(&mut rng);
        }
    }

    fn common_tiems() -> Vec<Item> {
        let mut items = Vec::new();

        items.push(Item::new(
            String::from("Double"),
            6,
            String::from("Double the ammount of snacks"),
            ItemType::Double,
        ));

        items.push(Item::new(
            String::from("Snacks"),
            7,
            String::from("Every Snack gives 2x"),
            ItemType::Snacks,
        ));

        items.push(Item::new(
            String::from("Foody"),
            5,
            String::from("For each food eaten this item will give 1 food for each time it has been triggerd."),
            ItemType::Foody,
        ));

        items.push(Item::new(
            String::from("Shedding"),
            6,
            String::from("For each food eaten 10% chance to remove some size"),
            ItemType::Shedding,
        ));

        items.push(Item::new(
            String::from("Snackception"),
            8,
            String::from("Every 5 snacks increases the value of a snack by 1"),
            ItemType::Snackception,
        ));

        return items;
    }

    fn rare_items() -> Vec<Item> {
        let mut items = Vec::new();

        // items.push(Item::new(
        //     String::from("Times (N/A)"),
        //     6,
        //     String::from("Every 20 snacks add 5 seconds to time"),
        //     ItemType::Time,
        // ));

        items.push(Item::new(
            String::from("GoldenSnack"),
            9,
            String::from("First snack gives 300 food"),
            ItemType::GoldenSnack,
        ));

        return items;
    }

    fn ultra_rare_items() -> Vec<Item> {
        let mut items = Vec::new();

        items.push(Item::new(
            String::from("Phantom Snake"),
            9,
            String::from("Snake Cannot collide with itself."),
            ItemType::PhantomSnake,
        ));
        items.push(Item::new(
            String::from("LongBoi"),
            9,
            String::from("Start with a snake length of 500."),
            ItemType::LongBoi,
        ));
        // items.push(Item::new(
        //     String::from("4Ever"),
        //     9,
        //     String::from("Prevent snake length reset on new rounds."),
        //     ItemType::ForEver,
        // ));

        return items;
    }
}
