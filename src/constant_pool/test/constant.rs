use bytes::Bytes;

use crate::{
    read_write::{Read, Write},
    Constant,
};

static TEST_CONSTANT_DATA: [u8; 16] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

#[test]
fn test_read_constant() {
    let mut bytes = Bytes::from(&TEST_CONSTANT_DATA[..]);
    let constant = Constant::read(&mut bytes);
    assert_eq!(constant.ty, 0x00);
    assert_eq!(constant.layout, 0x01);
    assert_eq!(constant.data_offset, 0x00);
}

#[test]
fn test_write_constant() {
    let constant = Constant {
        ty: 0x00,
        layout: 0x01,
        data_offset: 0x00,
    };
    let mut res = vec![];
    constant.write(&mut res);
    assert_eq!(res, Vec::from(TEST_CONSTANT_DATA));
}
