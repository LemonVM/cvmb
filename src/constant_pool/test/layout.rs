use bytes::Bytes;

use crate::{
    read_write::{Read, Write},
    Layout, LayoutBuilder,
};

static TEST_LAYOUT_DATA: [u8; 23] = [
    0x00, 0x00, 0xFF, 0xFF, 0x00, 0xEE, 0x00, 0x00, 0x00, 0x01, 0xFF, 0x00, 0x00, 0x00, 0x02, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
];

#[test]
fn test_read_layout() {
    let mut bytes = Bytes::from(&TEST_LAYOUT_DATA[..]);
    let layout = Layout::read(&mut bytes);
    assert_eq!(layout.id, 0xFFFF);
    assert_eq!(layout.length, 0xEE);
    assert_eq!(layout.field_length, vec![0xFF]);
    assert_eq!(layout.field_names, vec![0x00, 0x01])
}

#[test]
fn test_write_layout() {
    let mut res = vec![];
    let layout = Layout {
        id: 0xFFFF,
        length: 0xEE,
        field_length: vec![0xFF],
        field_names: vec![0x00, 0x01],
    };
    layout.write(&mut res);
    assert_eq!(res, Vec::from(TEST_LAYOUT_DATA))
}

fn exists<T: Eq>(x: &Vec<T>, cmp: &T) -> bool {
    x.iter().any(|x| x == cmp)
}

#[test]
fn test_layout_builder_case_no_repeat() {
    let builded = LayoutBuilder::new()
        .add_field(Some("gay1".into()), 10)
        .add_field(Some("gay2".into()), 14)
        .build();
    assert_eq!(builded.length, 24);
    assert!(exists(&builded.field_names, &Some("gay1".to_string())));
    assert!(exists(&builded.field_names, &Some("gay2".to_string())));
    assert!(exists(&builded.field_length, &10));
    assert!(exists(&builded.field_length, &14));
}

#[test]
fn test_layout_builder_case_repeat() {
    let builded = LayoutBuilder::new()
        .add_field(Some("gay".into()), 10)
        .add_field(Some("gay".into()), 10)
        .build();
    assert_eq!(builded.length, 10);
    assert_eq!(builded.field_names, vec![Some("gay".into())]);
    assert_eq!(builded.field_length, vec![10])
}
