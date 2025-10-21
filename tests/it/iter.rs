use crate::for_test::builders::*;
use crate::for_test::samples::sample_sv;
use sparse_vector::Iter;

#[test]
fn clone() {
    let vec = sample_sv::normal();
    let target = vec.iter();
    let result = target.clone();
    assert!(result.eq(vec.iter()));
}

#[test]
fn default() {
    let result = Iter::<i32>::default();
    assert_eq!(result.size_hint(), (0, Some(0)));
    assert_eq!(result.count(), 0);
}

#[test]
fn next() {
    with_empty();
    with_overrun();
    with_normal();
    with_all_padding();
    with_tail_passed();
    with_tail_memoed();

    fn with_empty() {
        let vec = sample_sv::default();
        let target = &mut vec.iter();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = sample_sv::normal();
        let target = &mut vec.iter();
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
            let target = &mut vec.iter();
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, builder.values().get(index));
        }
    }

    fn with_all_padding() {
        let builder = SparseVecBuilder::new().set_nnp(0);
        for index in builder.some_indexs() {
            // Arrange.
            let vec = builder.build();
            let target = &mut vec.iter();
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, builder.values().get(index));
        }
    }

    fn with_tail_passed() {
        let builder = SparseVecBuilder::new();
        for index in builder.some_indexs() {
            // Arrange.
            let vec = builder.build();
            let target = &mut vec.iter();
            let back_len = vec.len() - index;
            target.nth_back(back_len - 1);
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, None);
        }
    }

    fn with_tail_memoed() {
        let builder = SparseVecBuilder::new();
        let indexs = builder.some_indexs().into_iter();
        let indexs = indexs.filter(|x| *x < builder.len() - 1);
        for index in indexs {
            // Arrange vec.
            let vec = &mut builder.build();
            let tail_pos = index + 1;
            *vec.edit(tail_pos) = builder.padding();

            // Arrange iter.
            let target = &mut vec.iter();
            let back_len = vec.len() - tail_pos;
            target.nth_back(back_len - 1);
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, builder.values().get(index));
        }
    }
}

#[test]
fn size_hint() {
    let vec = sample_sv::normal();
    let target = vec.iter();
    let result = target.size_hint();
    assert_eq!(result, (vec.len(), Some(vec.len())));
}

#[test]
fn next_back() {
    with_empty();
    with_overrun();
    with_normal();
    with_all_padding();
    with_head_passed();
    with_head_memoed();

    fn with_empty() {
        let vec = sample_sv::default();
        let target = &mut vec.iter();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = sample_sv::normal();
        let target = &mut vec.iter();
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
            let target = &mut vec.iter();
            let back_len = builder.len() - index - 1;
            if back_len > 1 {
                target.nth_back(back_len - 1);
            }

            // Act.
            let result = target.next_back();

            // Assert.
            assert_eq!(result, builder.values().get(index));
        }
    }

    fn with_all_padding() {
        let builder = SparseVecBuilder::new().set_nnp(0);
        for index in builder.some_indexs() {
            // Arrange.
            let vec = builder.build();
            let target = &mut vec.iter();
            let back_len = builder.len() - index - 1;
            if back_len > 1 {
                target.nth_back(back_len - 1);
            }

            // Act.
            let result = target.next_back();

            // Assert.
            assert_eq!(result, builder.values().get(index));
        }
    }

    fn with_head_passed() {
        let builder = SparseVecBuilder::new();
        for index in builder.some_indexs() {
            // Arrange.
            let vec = builder.build();
            let target = &mut vec.iter();
            let back_len = vec.len() - index - 1;
            target.nth(index);
            if back_len > 0 {
                target.nth_back(back_len - 1);
            }

            // Act.
            let result = target.next_back();

            // Assert.
            assert_eq!(result, None);
        }
    }

    fn with_head_memoed() {
        let builder = SparseVecBuilder::new();
        let indexs = builder.some_indexs().into_iter();
        let indexs = indexs.filter(|x| *x > 0);
        for index in indexs {
            // Arrange vec.
            let vec = &mut builder.build();
            let head_pos = index - 1;
            *vec.edit(head_pos) = builder.padding();

            // Arrange iter.
            let target = &mut vec.iter();
            let back_len = vec.len() - index - 1;
            target.nth(head_pos);
            if back_len > 0 {
                target.nth_back(back_len - 1);
            }

            // Act.
            let result = target.next_back();

            // Assert.
            assert_eq!(result, builder.values().get(index));
        }
    }
}
