use crate::for_test::builders::*;
use crate::for_test::samples::*;
use std::mem;

#[test]
fn drop() {
    with_values_removed();
    with_values_remained();

    fn with_values_removed() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let range = range_for(builder.len()).include(builder.some_npad_indexs(1)[0]);
        let inserts = sparse_values(range.len() / 2);
        let mut target = vec.splice(range.clone(), inserts.clone());
        let _ = target.all(|_| true);

        // Act.
        mem::drop(target);

        // Assert.
        let rhs = &mut builder.values();
        rhs.splice(range.clone(), inserts.clone());
        assert_eq!(&vec.to_vec(), rhs);
    }

    fn with_values_remained() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let range = range_for(builder.len()).include(builder.some_npad_indexs(1)[0]);
        let inserts = sparse_values(range.len() / 2);
        let target = vec.splice(range.clone(), inserts.clone());

        // Act.
        mem::drop(target);

        // Assert.
        let rhs = &mut builder.values();
        rhs.splice(range.clone(), inserts.clone());
        assert_eq!(&vec.to_vec(), rhs);
    }
}

#[test]
fn next_back() {
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range_for(vec.len()).empty();
        let inserts = sparse_values(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range_for(vec.len()).normal();
        let inserts = sparse_values(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());
        target.nth_back(range.len() - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let range = range_for(vec.len()).normal();
        let inserts = sparse_values(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());
        let index = range.len() / 2;
        target.nth_back(range.len() - index - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, Some(builder.values()[range.start + index]));
    }
}

#[test]
fn next() {
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range_for(vec.len()).empty();
        let inserts = sparse_values(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range_for(vec.len()).normal();
        let inserts = sparse_values(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());
        target.nth(range.len() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let range = range_for(vec.len()).normal();
        let inserts = sparse_values(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());
        let index = range.len() / 2;
        target.nth(index - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, Some(builder.values()[range.start + index]));
    }
}

#[test]
fn size_hint() {
    // Arrange.
    let vec = &mut SparseVecSample::normal();
    let range = range_for(vec.len()).normal();
    let inserts = sparse_values(range.len() / 2);
    let target = &mut vec.splice(range.clone(), inserts.clone());

    // Act.
    let result = target.size_hint();

    // Assert.
    assert_eq!(result, (range.len(), Some(range.len())));
}
