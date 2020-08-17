## JSX parser in Rust


Simple and light JSX like syntax parser, written in Rust, building this as macro coding experiment.

Spews out `Node` struct with children and attributes attached.



* Examples 

```Rust
use cig::{rsx, Node};

let node: Node = rsx!(<test data={"entry"}></test>);

let node: Node = rsx!(<test data={"entry"}>
                        {
                            ["test", "test2"]
                            .iter()
                            .map(|x| {
                                rsx!(<node>
                                        <children data={x}></children>
                                    </node>)
                            })
                            .collect::<Vec<Node>>()
                        }
                        </test>);
                        
                        
                        
let click = || println!("{:?}", "data");                        
let node = rsx!(<test OnClick={ click } OnHover={click} data1={"test"} data2={"test2"}></test>);                        

```
