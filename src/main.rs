use tide::{App, Context};
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
