use crate::yaml::*;

#[test]
fn duplicate_map_key_in_root() {
    let yaml = r#"
a: 1
a: 1
"#;
    let yaml = crate::yaml::YamlLoader::load_from_str(yaml).expect("parsed yaml");

    assert_eq!(
        yaml.errors,
        vec![error::Error::DuplicateKey(error::DuplicateKey {
            key: YamlElt::String("a".to_owned()),
            first_mark: Marker {
                index: 4,
                line: 2,
                col: 3
            },
            first_value: YamlElt::Integer(1),
            second_mark: Marker {
                index: 9,
                line: 3,
                col: 3
            },
            second_value: YamlElt::Integer(1)
        })]
    )
}

#[test]
fn duplicate_map_key() {
    let yaml = r#"
root:
  a: 1
  a: 1
"#;
    let yaml = crate::yaml::YamlLoader::load_from_str(yaml).expect("parsed yaml");
    if !matches!(yaml.errors.as_slice(), [error::Error::DuplicateKey(_)]) {
        panic!("DuplicateKey error expected")
    }
}
