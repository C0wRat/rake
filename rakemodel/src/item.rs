#[derive(Debug, Clone)]
pub enum ItemType {
    Shears,
    Snacks,
    Double,
    Time,
    ForEver,
    LongBoi,
    PhantomSnake,
    Snackception,
    GoldenSnack,
    Foody,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub item_type: ItemType,
    pub item_name: String,
    pub value: i32,
    pub description: String,
    pub triggered: bool,
}

impl Item {
    pub fn new(item_name: String, value: i32, description: String, item_type: ItemType) -> Self {
        Self {
            item_name,
            value,
            description,
            item_type,
            triggered: false,
        }
    }
}
