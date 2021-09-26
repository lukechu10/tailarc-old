use crate::map::Map;

pub trait MapBuilder {
    fn build(new_depth: i32) -> Map;
}
