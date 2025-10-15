use crate::for_test::sample as ts;
use crate::for_test::template as tt;
use sparse_vector::Iter;

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
        let vec = ts::default();
        let target = &mut vec.iter();
        let result = target.next();
        assert_eq!(result, None);
    }

    fn with_overrun() {
        // Arrange.
        let vec = ts::normal();
        let target = &mut vec.iter();
        target.nth(target.len() - 1);

        // Act.
        let result = target.next();

        // Assert.
        assert_eq!(result, None);
    }

    fn with_normal() {
        let template = tt::template();
        let vec = template.build();
        for index in template.sample_indexs() {
            // Arrange.
            let target = &mut vec.iter();
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, template.sample_vec().get(index));
        }
    }

    fn with_all_padding() {
        let template = tt::template().set_nnp(0);
        let vec = template.build();
        for index in template.sample_indexs() {
            // Arrange.
            let target = &mut vec.iter();
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, template.sample_vec().get(index));
        }
    }

    fn with_tail_passed() {
        let template = tt::template();
        let vec = template.build();
        for index in template.sample_indexs() {
            // Arrange.
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
        let template = tt::template();
        let vec = &mut template.build();
        let indexs = template.sample_indexs().into_iter();
        let indexs = indexs.filter(|x| *x < template.len() - 1);
        for index in indexs {
            // Arrange.
            *vec.edit(index + 1) = template.padding();
            let target = &mut vec.iter();
            let back_len = vec.len() - (index + 1);
            target.nth_back(back_len - 1);
            if index > 0 {
                target.nth(index - 1);
            }

            // Act.
            let result = target.next();

            // Assert.
            assert_eq!(result, template.sample_vec().get(index));
        }
    }
}
