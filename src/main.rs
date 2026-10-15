//          ╭─────────────────────────────────────────────────────────╮
//          │                    BOX SMART POINTER                    │
//          ╰─────────────────────────────────────────────────────────╯

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    /*
    let x = 0.625;
    let y = Box::new(x);
    let z = &x;
    */
    let list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );
    println!("list => {:?}", list);
}
