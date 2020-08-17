## JSX parser in Rust


Simple and light JSX like syntax parser, written in Rust.

Spews out `Node` struct with children and attributes attached.



* Examples 

```Rust
let node: Node = rsx!(<test data={"entry"}></test>);

```



```Rust
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
