use crate::{
    read_write::{Read, Write},
    Constant, ConstantPool, ConstantPoolBuilder, Layout, LayoutBuilder, Type, TypeBuilder,
};

pub mod constant;
pub mod layout;
pub mod ty;

#[test]
fn test_constant_pool_builder_read_write() {
    let int_layout = LayoutBuilder::new().add_field(None, 4).build();
    let int_layout_2 = LayoutBuilder::new()
        .add_field(Some("Int".to_string()), 4)
        .build();
    let int_layout_3 = LayoutBuilder::new()
        .add_field(Some("Int".to_string()), 3)
        .build();
    let int = TypeBuilder::new()
        .set_name("Int".to_string())
        .add_layout(&int_layout)
        .add_layout(&int_layout_2)
        .add_layout(&int_layout_3)
        .build();
    let pool = ConstantPoolBuilder::new()
        .add_constants(&int, &int_layout, vec![0x00, 0x00, 0x00, 0x01])
        .add_constants(&int, &int_layout_2, vec![0x00, 0x00, 0x00, 0x01])
        .add_constants(&int, &int_layout_3, vec![b'I', b'n', b't'])
        .build();

    // test read write
    let mut bytes = vec![];
    pool.clone().write(&mut bytes);
    let pool2 = ConstantPool::read(&mut bytes.as_slice());
    assert_eq!(pool, pool2);

    // test ziping
    assert_eq!(
        pool.string_pool,
        vec![b'I', b'n', b't', 0x00, 0x00, 0x00, 0x01]
    );
    assert_eq!(
        pool.constants,
        vec![
            Constant {
                ty: 0,
                layout: 0,
                data_offset: 3
            },
            Constant {
                ty: 0,
                layout: 1,
                data_offset: 3
            },
            Constant {
                ty: 0,
                layout: 2,
                data_offset: 0
            }
        ]
    );
    assert_eq!(
        pool.layouts,
        vec![
            Layout {
                id: 0,
                length: 4,
                field_length: vec![4],
                field_names: vec![4294967295]
            },
            Layout {
                id: 1,
                length: 4,
                field_length: vec![4],
                field_names: vec![0]
            },
            Layout {
                id: 2,
                length: 3,
                field_length: vec![3],
                field_names: vec![0]
            }
        ]
    );
    assert_eq!(
        pool.types,
        vec![Type {
            id: 0,
            name: 0,
            possible_layouts: vec![0, 1, 2]
        }]
    );
}
