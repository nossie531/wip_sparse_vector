use crate::for_test::sample as ts;
use crate::for_test::template as tt;
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
        let vec = ts::default();
        let target = &mut vec.into_iter();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = ts::normal();
        let target = &mut vec.into_iter();
        target.nth(target.len() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        let template = tt::template();
        for index in template.sample_indexs() {
            // Arrange.
            let vec = template.build();
            let target = &mut vec.into_iter();
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, Some(template.sample_vec()[index]));
        }
    }
}

#[test]
fn size_hint() {
    let template = tt::template();
    let vec = template.build();
    let target = vec.into_iter();
    let result = target.size_hint();
    assert_eq!(result, (template.len(), Some(template.len())));
}

#[test]
fn next_back() {
    with_empty();
    with_overrun();
    with_normal();

    fn with_empty() {
        let vec = ts::default();
        let target = &mut vec.into_iter();
        let result = target.next_back();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = ts::normal();
        let target = &mut vec.into_iter();
        target.nth_back(target.len() - 1);

        // Act.
        let result = target.next_back();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        let template = tt::template();
        for index in template.sample_indexs() {
            // Arrange.
            let vec = template.build();
            let target = &mut vec.into_iter();
            let back_len = template.len() - index - 1;
            if back_len > 1 {
                target.nth_back(back_len - 1);
            }

            // Act.
            let result = target.next_back();

            // Assert.
            assert_eq!(result, Some(template.sample_vec()[index]));
        }
    }
}
