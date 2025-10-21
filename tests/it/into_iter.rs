use crate::for_test::builders::*;
use crate::for_test::samples::sample_sv;
use sparse_vector::IntoIter;

#[test]
fn default() {
    let result = IntoIter::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}

#[test]
fn next() {
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        let vec = sample_sv::default();
        let target = &mut vec.into_iter();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = sample_sv::normal();
        let target = &mut vec.into_iter();
        target.nth(target.len() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        let builder = SparseVecBuilder::new();
        for index in builder.some_indexs() {
            // Arrange.
            let vec = builder.build();
            let target = &mut vec.into_iter();
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, Some(builder.values()[index]));
        }
    }
}

#[test]
fn size_hint() {
    let builder = SparseVecBuilder::new();
    let vec = builder.build();
    let target = vec.into_iter();
    let result = target.size_hint();
    assert_eq!(result, (builder.len(), Some(builder.len())));
}

#[test]
fn next_back() {
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        let vec = sample_sv::default();
        let target = &mut vec.into_iter();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = sample_sv::normal();
        let target = &mut vec.into_iter();
        target.nth_back(target.len() - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        let builder = SparseVecBuilder::new();
        for index in builder.some_indexs() {
            // Arrange.
            let vec = builder.build();
            let target = &mut vec.into_iter();
            let back_len = builder.len() - index - 1;
            if back_len > 1 {
                target.nth_back(back_len - 1);
            }

            // Act.
            let result = target.next_back();

            // Assert.
            assert_eq!(result, Some(builder.values()[index]));
        }
    }
}
