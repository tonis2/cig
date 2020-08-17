use cig::{rsx, Node};

fn main() {
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
    println!("{:?}", node);
}
