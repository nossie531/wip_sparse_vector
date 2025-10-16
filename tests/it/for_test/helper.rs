use sparse_vector::SparseWriter;
use std::hash::{DefaultHasher, Hash, Hasher};

pub fn vec_from_sparse_writer<'a>(sw: &mut SparseWriter<'a, i32>) -> Vec<(usize, i32)> {
    let mut ret = Vec::new();
    while let Some(elm) = sw.next() {
        ret.push((elm.index(), *elm.value()));
    }

    ret
}

pub fn hash<T: Hash>(target: &T) -> u64 {
    let hasher = &mut DefaultHasher::new();
    target.hash(hasher);
    hasher.finish()
}
