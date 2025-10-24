use crate::for_test::builders::ValuesBuilder;

pub struct VecSample();

impl VecSample {
    pub fn normal(len: usize) -> Vec<i32> {
        let builders = ValuesBuilder::new().set_len(len);
        builders.values()
    }
}
