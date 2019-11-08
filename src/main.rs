use std::ops::Add;
use std::rc::Rc;
#[derive(Debug)]
enum List<T: Add<Output = T>> {
    Cons(T, Rc<List<T>>),
    Undefined,
}

use List::{Cons, Undefined};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Undefined)));
    // b is shared owner of a
    let b = Rc::new(Cons(3, Rc::clone(&a)));
    // c is shared owner of a
    let c = Rc::new(Cons(4, Rc::clone(&a)));

    println!("{:#?}-{:#?}-{:#?}", *a, *b, *c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn rc_test() {
        let a = Rc::new(Cons(5, Rc::new(Undefined)));
        assert_eq!(1, Rc::strong_count(&a));
        {
            let _b = Rc::new(Cons(3, Rc::clone(&a)));
            assert_eq!(2, Rc::strong_count(&a));
        }
        let _c = Rc::new(Cons(4, Rc::clone(&a)));
        assert_eq!(2, Rc::strong_count(&a));
    }
}
