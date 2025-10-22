use crate::for_test::builders::ValuesBuilder;

pub fn normal(len: usize) -> Vec<i32> {
    let builders = ValuesBuilder::new().set_len(len);
    builders.values()
}