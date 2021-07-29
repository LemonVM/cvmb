use bytes::Bytes;

use crate::{
    read_write::{Read, Write},
    BuildedLayout, LayoutBuilder, Type, TypeBuilder,
};

static TEST_TYPE_DATA: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFF, 0xFF, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0xEE, 0xEE,
];

#[test]
fn test_read_ty() {
    let mut bytes = Bytes::from(&TEST_TYPE_DATA[..]);
    let ty = Type::read(&mut bytes);
    assert_eq!(ty.id, 0x00);
    assert_eq!(ty.name, 0xFFFF);
    assert_eq!(ty.possible_layouts, vec![0xEEEE])
}

#[test]
fn test_write_ty() {
    let ty = Type {
        id: 0x00,
        name: 0xFFFF,
        possible_layouts: vec![0xEEEE],
    };
    let mut res = vec![];
    ty.write(&mut res);
    assert_eq!(res, Vec::from(TEST_TYPE_DATA));
}

#[test]
fn test_ty_builder() {
    let builded = TypeBuilder::new()
        .set_name("Gay".to_string())
        .add_layout(
            &LayoutBuilder::new()
                .add_field(Some("gay".to_string()), 114)
                .build(),
        )
        .build();
    assert_eq!(builded.name, Some("Gay".to_string()));
    assert_eq!(
        builded.layouts,
        vec![BuildedLayout {
            length: 114,
            field_names: vec![Some("gay".to_string())],
            field_length: vec![114]
        }]
    );
}
