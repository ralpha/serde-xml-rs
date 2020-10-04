#[macro_use]
extern crate serde_derive;
extern crate serde_xml_rs;

extern crate log;
extern crate simple_logger;

use serde_xml_rs::from_str;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[test]
fn simple_struct_from_attributes() {
    let _ = simple_logger::init();

    let s = r##"
        <item name="hello" source="world.rs" />
    "##;

    let item: Item = from_str(s).unwrap();

    assert_eq!(
        item,
        Item {
            name: "hello".to_string(),
            source: "world.rs".to_string(),
        }
    );
}

#[test]
fn multiple_roots_attributes() {
    let _ = simple_logger::init();

    let s = r##"
        <item name="hello" source="world.rs" />
        <item name="hello" source="world.rs" />
    "##;

    let item: Vec<Item> = from_str(s).unwrap();

    assert_eq!(
        item,
        vec![
            Item {
                name: "hello".to_string(),
                source: "world.rs".to_string(),
            },
            Item {
                name: "hello".to_string(),
                source: "world.rs".to_string(),
            },
        ]
    );
}

#[test]
fn simple_struct_from_attribute_and_child() {
    let _ = simple_logger::init();

    let s = r##"
        <item name="hello">
            <source>world.rs</source>
        </item>
    "##;

    let item: Item = from_str(s).unwrap();

    assert_eq!(
        item,
        Item {
            name: "hello".to_string(),
            source: "world.rs".to_string(),
        }
    );
}

#[derive(Debug, Deserialize, PartialEq)]
struct Project {
    name: String,

    #[serde(rename = "item", default)]
    items: Vec<Item>,
}

#[test]
fn nested_collection() {
    let _ = simple_logger::init();

    let s = r##"
        <project name="my_project">
            <item name="hello1" source="world1.rs" />
            <item name="hello2" source="world2.rs" />
        </project>
    "##;

    let project: Project = from_str(s).unwrap();

    assert_eq!(
        project,
        Project {
            name: "my_project".to_string(),
            items: vec![
                Item {
                    name: "hello1".to_string(),
                    source: "world1.rs".to_string(),
                },
                Item {
                    name: "hello2".to_string(),
                    source: "world2.rs".to_string(),
                },
            ],
        }
    );
}

#[derive(Debug, Deserialize, PartialEq)]
enum MyEnum {
    A(String),
    B { name: String, flag: bool },
    C,
}

#[derive(Debug, Deserialize, PartialEq)]
struct MyEnums {
    #[serde(rename = "$value")]
    items: Vec<MyEnum>,
}

#[test]
fn collection_of_enums() {
    let _ = simple_logger::init();

    let s = r##"
        <enums>
            <A>test</A>
            <B name="hello" flag="true" />
            <C />
        </enums>
    "##;

    let project: MyEnums = from_str(s).unwrap();

    assert_eq!(
        project,
        MyEnums {
            items: vec![
                MyEnum::A("test".to_string()),
                MyEnum::B {
                    name: "hello".to_string(),
                    flag: true,
                },
                MyEnum::C,
            ],
        }
    );
}

#[test]
fn out_of_order_collection() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Collection {
        a: Vec<A>,
        b: Vec<B>,
        c: C,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct A {
        name: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct B {
        name: String,
    }

    #[derive(Debug, Deserialize, PartialEq)]
    struct C {
        name: String,
    }

    let _ = simple_logger::init();

    let in_xml = r#"
        <collection>
            <a name="a1" />
            <a name="a2" />
            <b name="b1" />
            <a name="a3" />
            <c name="c" />
            <b name="b2" />
            <a name="a4" />
        </collection>
    "#;

    let should_be = Collection {
        a: vec![
            A { name: "a1".into() },
            A { name: "a2".into() },
            A { name: "a3".into() },
            A { name: "a4".into() },
        ],
        b: vec![B { name: "b1".into() }, B { name: "b2".into() }],
        c: C { name: "c".into() },
    };

    let actual: Collection = from_str(&in_xml).unwrap();

    assert_eq!(should_be, actual);
}
