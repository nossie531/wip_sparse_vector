use crate::for_test::builders::*;
use crate::for_test::range;
use crate::for_test::samples::*;

#[test]
fn drop() {
    // TODO:
}

#[test]
fn next() {
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range::empty(vec.len());
        let inserts = VecSample::normal(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range::normal(vec.len());
        let inserts = VecSample::normal(range.len() / 2);
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
        let range = range::normal(vec.len());
        let inserts = VecSample::normal(range.len() / 2);
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
fn next_back() {
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range::empty(vec.len());
        let inserts = VecSample::normal(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = &mut SparseVecSample::normal();
        let range = range::normal(vec.len());
        let inserts = VecSample::normal(range.len() / 2);
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
        let range = range::normal(vec.len());
        let inserts = VecSample::normal(range.len() / 2);
        let target = &mut vec.splice(range.clone(), inserts.clone());
        let index = range.len() / 2;
        target.nth_back(range.len() - index - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, Some(builder.values()[range.start + index]));
    }
}
