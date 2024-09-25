const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;
fn main() {
    let missiles = STARTING_MISSILES;
    let ready = READY_AMOUNT - 1;
    // let mut missiles = 8;
    // let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
//    missiles = missiles - ready;
    println!("{} missiles left", missiles - ready);
}


fn do_stuff(qty: f64, oz: f64) -> f64 {
    qty * oz
}

// using Enums.
enum MyType {
    Integer(i32),
    Float(f64),
    Text(String),
}

fn my_function(arg: MyType) {
    match arg {
        MyType::Integer(i) => println!("Integer: {}", i),
        MyType::Float(f) => println!("Float: {}", f),
        MyType::Text(s) => println!("Text: {}", s),
    }
}