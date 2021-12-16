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

#[test]
fn merge_keys() {
    let yaml = r#"
- &OTHER { x: 1, y: 2000 }
- &OTHER { x: 1, y: 2 }

- x: 1000
  << : *OTHER
  << : *OTHER
"#;

    let mut expected_hash = LinkedHashMap::new();
    expected_hash.insert(
        Yaml {
            yaml: YamlElt::String("x".to_owned()),
            marker: Marker {
                index: 52,
                line: 5,
                col: 2,
            },
        },
        Yaml {
            yaml: YamlElt::Integer(1000),
            marker: Marker {
                index: 55,
                line: 5,
                col: 5,
            },
        },
    );
    expected_hash.insert(
        Yaml {
            yaml: YamlElt::String("y".to_owned()),
            marker: Marker {
                index: 42,
                line: 3,
                col: 17,
            },
        },
        Yaml {
            yaml: YamlElt::Integer(2),
            marker: Marker {
                index: 45,
                line: 3,
                col: 20,
            },
        },
    );

    let yaml = crate::yaml::YamlLoader::load_from_str(yaml).expect("parsed yaml");
    assert_eq!(yaml.errors, vec![]);

    match &yaml.docs[0].yaml {
        YamlElt::Array(v) => {
            assert_eq!(v[2].yaml, crate::yaml::YamlElt::Hash(expected_hash))
        }
        _ => panic!("Unexpected root element"),
    }
}
