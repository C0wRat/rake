use crate::grid::GridObject;


#[derive(Clone, Debug)]
pub struct food{
    pub body: GridObject,
}

impl food{
    pub fn new(x: i32, y: i32, value: i32, icon: char) -> Self{
        let body = GridObject::new(x, y, icon, crate::grid::ObjectType::Food(value), None);
        Self { body }
    }
}