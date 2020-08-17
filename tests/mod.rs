use cig::{rsx, Node};

#[test]
fn parse_block_children() {
    let children: Vec<Node> = ["test", "test2"]
        .iter()
        .map(|_x| {
            rsx!(<node>
                    <children></children>
                 </node>)
        })
        .collect();

    let node: Node = rsx!(<test>
                            <element></element>
                            { children }
                        </test>);

    assert_eq!(node.children.len(), 3);
}

#[test]
fn parse_regular_children() {
    let node: Node = rsx!(<test>
                            <child></child>
                        </test>);

    assert_eq!(node.children.len(), 1);
}

#[test]
fn parse_single_node() {
    let node: Node = rsx!(<test></test>);

    assert_eq!(node.tag, "test".to_string());
    assert_eq!(node.children.len(), 0);
}

#[test]
fn plain_attribute() {
    let node: Node = rsx!(<test data={"entry"}></test>);
    assert_eq!(*node.get_attribute("data").unwrap(), "entry".to_string());
}

#[test]
fn child_attribute() {
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

    assert_eq!(*node.get_attribute("data").unwrap(), "entry".to_string());
    assert_eq!(
        *node.children[0].children[0].get_attribute("data").unwrap(),
        "test".to_string()
    );
    assert_eq!(
        *node.children[0].children[1].get_attribute("data").unwrap(),
        "test2".to_string()
    );
}
