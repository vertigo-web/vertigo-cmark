use pulldown_cmark::MetadataBlockKind;
use vertigo::{
    dev::inspect::{DomDebugFragment, log_start},
    dom,
};

use crate::{Metadata, metadata, to_vertigo_with_metadata};

#[test]
fn a_yaml_block_is_read_apart() {
    let document = "---\ntitle: The cave\nera: 1\n---\n\n# The cave\n";
    assert_eq!(
        metadata(document),
        Some(Metadata {
            kind: MetadataBlockKind::YamlStyle,
            text: "title: The cave\nera: 1\n".to_string(),
        })
    );
}

#[test]
fn a_toml_block_is_read_apart() {
    let document = "+++\nid = \"cave\"\nsources = [\"RB 1\"]\n+++\nText\n";
    assert_eq!(
        metadata(document),
        Some(Metadata {
            kind: MetadataBlockKind::PlusesStyle,
            text: "id = \"cave\"\nsources = [\"RB 1\"]\n".to_string(),
        })
    );
}

#[test]
fn only_the_beginning_holds_metadata() {
    assert_eq!(metadata("# Title\n\n---\ntitle: no\n---\n"), None);
    assert_eq!(metadata("Just text"), None);
    assert_eq!(metadata(""), None);
}

#[test]
fn the_block_is_not_rendered() {
    log_start();
    let (block, _el1) = to_vertigo_with_metadata("---\ntitle: The cave\n---\n\nText\n");
    let el1_str = DomDebugFragment::from_log().to_pseudo_html();

    log_start();
    let _el2 = dom! {
        <div>
            <p>"Text"</p>
        </div>
    };
    let el2_str = DomDebugFragment::from_log().to_pseudo_html();

    assert_eq!(el1_str, el2_str);
    assert_eq!(
        block.map(|block| block.text),
        Some("title: The cave\n".to_string())
    );
}
