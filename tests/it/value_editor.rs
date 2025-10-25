use crate::for_test::builders::*;
use crate::for_test::helper;
use std::mem;
use std::ops::{Deref, DerefMut};

#[test]
fn deref() {
    with_normal();
    with_padding();
    with_new_value();

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_npad_indexs(1)[0];
        let target = vec.edit(index);

        // Act.
        let result = target.deref();

        // Assert.
        assert_eq!(*result, builder.values()[index]);
    }

    fn with_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_pad_indexs(1)[0];
        let target = vec.edit(index);

        // Act.
        let result = target.deref();

        // Assert.
        assert_eq!(*result, builder.values()[index]);
    }

    fn with_new_value() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_npad_indexs(1)[0];
        let mut target = vec.edit(index);
        let new_value = builder.none_padding();
        *target = new_value;

        // Act.
        let result = target.deref();

        // Assert.
        assert_eq!(*result, new_value);
    }
}

#[test]
fn deref_mut() {
    with_normal();
    with_padding();
    with_new_value();

    fn with_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_npad_indexs(1)[0];
        let target = &mut vec.edit(index);

        // Act.
        let result = target.deref_mut();

        // Assert.
        assert_eq!(*result, builder.values()[index]);
    }

    fn with_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_pad_indexs(1)[0];
        let target = &mut vec.edit(index);

        // Act.
        let result = target.deref_mut();

        // Assert.
        assert_eq!(*result, builder.values()[index]);
    }

    fn with_new_value() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_npad_indexs(1)[0];
        let mut target = vec.edit(index);
        let new_value = builder.none_padding();
        *target = new_value;

        // Act.
        let result = target.deref_mut();

        // Assert.
        assert_eq!(*result, new_value);
    }
}

#[test]
fn drop() {
    with_no_edit();
    with_normal_to_normal();
    with_normal_to_padding();
    with_padding_to_normal();

    fn with_no_edit() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_npad_indexs(1)[0];
        let target = vec.edit(index);

        // Act.
        mem::drop(target);

        // Assert.
        assert_eq!(vec[index], builder.values()[index]);
    }

    fn with_normal_to_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_npad_indexs(1)[0];
        let mut target = vec.edit(index);
        let value = helper::some_other_of([*target, builder.padding()]);
        *target = value;

        // Act.
        mem::drop(target);

        // Assert.
        assert_eq!(vec[index], value);
    }

    fn with_normal_to_padding() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_npad_indexs(1)[0];
        let mut target = vec.edit(index);
        let value = builder.padding();
        *target = value;

        // Act.
        mem::drop(target);

        // Assert.
        assert_eq!(vec[index], value);
    }

    fn with_padding_to_normal() {
        // Arrange.
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let index = builder.some_pad_indexs(1)[0];
        let mut target = vec.edit(index);
        let value = builder.none_padding();
        *target = value;

        // Act.
        mem::drop(target);

        // Assert.
        assert_eq!(vec[index], value);
    }
}

#[test]
fn debug() {
    with_normal();
    with_padding();

    fn with_normal() {
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let target = vec.edit(builder.some_npad_indexs(1)[0]);
        let _ = format!("{:?}", target);
    }

    fn with_padding() {
        let builder = SparseVecBuilder::new();
        let vec = &mut builder.build();
        let target = vec.edit(builder.some_pad_indexs(1)[0]);
        let _ = format!("{:?}", target);
    }
}
