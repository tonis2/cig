use cig::{rsx, Node};

#[test]
fn parse_children() {
    let node: Node = rsx!(<test data={"entry"}>
                            {
                                ["test", "test2"]
                                .iter()
                                .map(|x| {
                                    rsx!(<node>
                                            <children default={"default"} data={x}></children>
                                        </node>)
                                })
                                .collect::<Vec<Node>>()
                            }
                         </test>);

    assert_eq!(*node.get_attribute("data").unwrap(), "entry".to_string());

    assert_eq!(node.children[0].tag, "node".to_string());

    assert_eq!(node.children[1].tag, "node".to_string());

    assert_eq!(node.children[1].children[0].tag, "children".to_string());

    assert_eq!(
        *node.children[1].children[0].get_attribute("data").unwrap(),
        "test2".to_string()
    );

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

    let node: Node = rsx!(<test>
                            <child>
                                <element></element>
                            </child>
                            <text></text>
                        </test>);

    assert_eq!(node.children.len(), 2);

    assert_eq!(node.children[0].children.len(), 1);
}

#[test]
fn self_closing_tag() {
    let node: Node = rsx!(<test>
                            <child/>
                            <text/>
                            <element>
                                <image/>
                                <image/>
                            </element>
                        </test>);

    assert_eq!(node.children.len(), 3);
    assert_eq!(node.children[2].children.len(), 2);
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


