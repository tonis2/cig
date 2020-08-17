use cig::{rsx, Events, Node};

fn main() {
    let click = || println!("{:?}", "data");
    let node = rsx!(<test OnClick={ click } OnHover={click} data1={"test"} data2={"test2"}></test>);
    println!("{:?}", node);
}
