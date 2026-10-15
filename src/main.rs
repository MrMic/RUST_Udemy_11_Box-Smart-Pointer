//          ╭─────────────────────────────────────────────────────────╮
//          │                    BOX SMART POINTER                    │
//          ╰─────────────────────────────────────────────────────────╯
/*
#[derive(Debug)]
enum List {
    Cons(i32, Option<Box<List>>),
}

fn main() {
    /*
    let x = 0.625;
    let y = Box::new(x);
    let z = &x;
    */
    let list = List::Cons(
        1,
        Some(Box::new(List::Cons(2, Some(Box::new(List::Cons(3, None)))))),
    );
    println!("list => {:?}", list);
}
*/

struct Huge_Data;
struct Small_Data;

trait Storage {}

impl Storage for Huge_Data {}
impl Storage for Small_Data {}

fn main() {
    let data1 = Huge_Data;
    let data2 = Box::new(Huge_Data);

    let data3 = data1; // data1 is moved to data3, data1 is no longer valid
    let data4 = data2; // data2 is moved to data4, data2 is no longer valid

    let data5 = Box::new(Small_Data);

    let data: Vec<Box<dyn Storage>> = vec![Box::new(data3), data4, data5]; // data3, data4, and data5 are moved into the vector
}
