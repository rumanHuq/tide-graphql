/* use tide::{App, Context};
fn main() {
    let mut app = App::new();

    app.at("/").get(root_service);

    app.serve("127.0.0.1:80").unwrap();
}

async fn root_service(_: Context<()>)->String{
    let ruman = Person;
    format!("I am smartass, {} that", ruman.smart())
}


struct Person;
trait Human {
    fn smart(&self)->bool;
}

impl Human for Person {
    fn smart(&self)->bool {
        true
    }
}



enum List {
    Cons(i32, Box<List>),
    Nil
}
 */

use std::ops::Deref;

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(item: T) -> MyBox<T> {
        MyBox(item)
    }
}

type Target<T> = T;
impl<T> Deref for MyBox<T> {
    type Target = Target<T>;
    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let c = MyBox::new("Hello");
    println!("{:?}", *c);
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn deref_test() {
        let a = 10;
        let b = MyBox::new(a * 10);

        println!("{:?}", *b);
        assert_eq!(10, a);
        // magick behind the scene
        // b.deref() = &self.0 -> *(&self.0) -> 100
        assert_eq!(100, *b);
    }
}
