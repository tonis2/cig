## JSX parser in Rust


Simple and light JSX like syntax parser, written in Rust.

Spews out `Node` struct with children and attributes attached.



* Examples 

```Rust
use cig::{rsx, Node};

let node: Node = rsx!(<test data={"entry"}></test>);

```



```Rust
use cig::{rsx, Node};

let node: Node = rsx!(<container data={"entry"}>
                        {
                            ["test", "test2"]
                            .iter()
                            .map(|x| {
                                rsx!(<node>
                                        <children data={x}/>
                                        <image/>
                                        <text/>
                                    </node>)
                            })
                            .collect::<Vec<Node>>()
                        }
                        </container>);

```
