use sparse_vector::iter::SparseWriter;

pub fn vec_from_sparse_writer<'a>(sw: &mut SparseWriter<'a, i32>) -> Vec<(usize, i32)> {
    let mut ret = Vec::new();
    while let Some(elm) = sw.next() {
        ret.push((elm.index(), *elm.value()));
    }

    ret
}
