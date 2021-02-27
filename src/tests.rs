use super::to_vertigo;

use vertigo::VDomNode;

#[test]
fn it_works() {
    let el = to_vertigo("Blabla");

    assert_eq!(el.children.len(), 1);

    let child = &el.children[0];

    match child {
        VDomNode::Element { node } => {
            assert_eq!(node.name, "p");
            assert_eq!(node.children.len(), 1);
            let text = &node.children[0];
            match text {
                VDomNode::Text { node } => {
                    assert_eq!(node.value, "Blabla");
                },
                _ => panic!("Invalid node type")
            }
        },
        _ => panic!("Invalid node type")
    }
}
