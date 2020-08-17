## JSX parser in Rust


Simple and light JSX like syntax parser, written in Rust, building this as macro coding experiment.

Spews out `Node` struct with children and attributes attached.



* Examples 

```Rust
use cig::{rsx, Node};

let node: Node = rsx!(<test data={"entry"}></test>);

```



```Rust
use cig::{rsx, Node};

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

```
