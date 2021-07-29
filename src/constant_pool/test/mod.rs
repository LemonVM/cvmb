use crate::{ConstantPool, ConstantPoolBuilder, LayoutBuilder, TypeBuilder, read_write::{Read, Write}};

pub mod constant;
pub mod layout;
pub mod ty;

#[test]
fn test_constant_pool_builder_read_write() {
    let int_layout = LayoutBuilder::new().add_field(None, 4).build();
    let int = TypeBuilder::new()
        .set_name("Int".to_string())
        .add_layout(int_layout.clone())
        .build();
    let pool = ConstantPoolBuilder::new()
        .add_constants(int, int_layout, vec![0x00, 0x00, 0x00, 0x01])
        .build();
    let mut bytes = vec![];
    pool.clone().write(&mut bytes);
    let pool2 = ConstantPool::read(&mut bytes.as_slice());
    assert_eq!(pool,pool2);
}
