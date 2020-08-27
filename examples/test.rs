use cig::{rsx, Node};

fn main() {
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
    for node in node.flat() {
        println!("{:?}", node.tag);
    }
}
