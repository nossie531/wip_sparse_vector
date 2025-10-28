use crate::for_test::builders::*;

#[test]
fn next() {
    with_normal();
    with_overrun();

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let writer = vec.sparse_writer();
        let target = &mut writer.map(|x| (x.0, *x.1));
        let n = builder.nnp() / 2;
        target.nth(n - 1);

        // Act.
        let result = target.next();

        // Assert.
        let rhs_idx = *builder.npad_indexs().iter().nth(n).unwrap();
        let rhs_val = builder.values()[rhs_idx];
        assert_eq!(result, Some((rhs_idx, rhs_val)));
    }

    fn with_overrun() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let writer = vec.sparse_writer();
        let target = &mut writer.map(|x| (x.0, *x.1));
        let n = builder.nnp();
        target.nth(n - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }
}

#[test]
fn next_back() {
    with_normal();
    with_overrun();

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let writer = vec.sparse_writer();
        let target = &mut writer.map(|x| (x.0, *x.1));
        let n = builder.nnp() / 2;
        target.nth_back(n - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        let rhs_idx = *builder.npad_indexs().iter().nth_back(n).unwrap();
        let rhs_val = builder.values()[rhs_idx];
        assert_eq!(result, Some((rhs_idx, rhs_val)));
    }

    fn with_overrun() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let writer = vec.sparse_writer();
        let target = &mut writer.map(|x| (x.0, *x.1));
        let n = builder.nnp();
        target.nth_back(n - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }
}
