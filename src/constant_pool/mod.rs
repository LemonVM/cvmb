use crate::read_write::{read_vec, write_vec, Read, Write};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fmt::Debug,
};

#[cfg(test)]
mod test;

pub mod constant;
pub mod layout;
pub mod ty;
pub use constant::*;
pub use layout::*;
pub use ty::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConstantPool {
    /// the data section
    string_pool: Vec<u8>,
    constants: Vec<Constant>,
    layouts: Vec<Layout>,
    types: Vec<Type>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstantPoolBuilder {
    types: HashSet<TypeBuilder>,
    constants: HashSet<(TypeBuilder, BuildedLayout, Vec<u8>)>,
}
impl ConstantPoolBuilder {
    pub fn new() -> Self {
        Self {
            types: HashSet::new(),
            constants: HashSet::new(),
        }
    }
    pub fn add_type(mut self, ty: TypeBuilder) -> Self {
        self.types.insert(ty);
        self
    }
    /// add constant if type exist, else add both type and constant
    pub fn add_constants(
        mut self,
        ty: &TypeBuilder,
        layout: &BuildedLayout,
        constant: Vec<u8>,
    ) -> Self {
        match self.types.get(&ty) {
            Some(_) => {
                self.constants
                    .insert((ty.clone(), layout.clone(), constant));
            }
            None => {
                self.types.insert(ty.clone());
                self.constants
                    .insert((ty.clone(), layout.clone(), constant));
            }
        };
        self
    }

    pub fn build(self) -> ConstantPool {
        let mut dup_string_size: u64 = 0;
        let mut strings = HashSet::new();
        let mut strings_with_conters = HashMap::new();

        let mut layouts = vec![];
        let mut types = vec![];
        let mut constants = vec![];
        let mut string_pool = vec![];
        let mut builded_layout_ids = HashMap::new();
        let mut type_builder_type_with_ids = HashMap::new();

        // returns the offset
        let mut add_string = |mut str: Vec<u8>| match strings.get(&str) {
            Some(key) => {
                dup_string_size += str.len() as u64;
                *strings_with_conters.get(key).unwrap()
            }
            None => {
                let res = string_pool.len() as u64;
                // remove duplicates
                strings.insert(str.clone());
                strings_with_conters.insert(str.clone(), res);
                string_pool.append(&mut str);
                res
            }
        };

        let mut add_symbol = |name: Option<String>| match name {
            Some(name) => add_string(name.as_bytes().to_vec()),
            None => 0xFFFFFFFF,
        };

        for ty in self.types {
            let name = add_symbol(ty.name.clone()) as u32;
            let start_layouts_len = layouts.len() as u32;
            for BuildedLayout {
                length,
                field_names,
                field_length,
            } in &ty.layouts
            {
                let mut field_ids = Vec::with_capacity(field_length.len());
                for field_name in field_names {
                    field_ids.push(add_symbol(field_name.clone()) as u32);
                }
                let layout = Layout {
                    id: layouts.len() as u32,
                    length: *length,
                    field_length: field_length.clone(),
                    field_names: field_ids,
                };
                layouts.push(layout);
                builded_layout_ids.insert(
                    BuildedLayout {
                        length: *length,
                        field_names: field_names.clone(),
                        field_length: field_length.clone(),
                    },
                    layouts.len() - 1,
                );
            }
            let end_layouts_len = layouts.len() as u32;
            let final_type = Type {
                id: types.len() as u32,
                name,
                possible_layouts: (start_layouts_len..end_layouts_len)
                    .into_iter()
                    .collect::<Vec<_>>(),
            };
            types.push(final_type);
            type_builder_type_with_ids.insert(ty, types.len() as u32 - 1);
        }

        for cont in self.constants {
            let ty = *type_builder_type_with_ids.get(&cont.0).unwrap();
            let layout = *builded_layout_ids.get(&cont.1).unwrap() as u32;

            // duplicate string removal
            let data_offset = add_string(cont.2);

            let constant = Constant {
                ty,
                layout,
                data_offset,
            };
            constants.push(constant);
        }
        constants.sort_by(|a, b| {
            let cmp_ty = a.ty.cmp(&b.ty);
            if cmp_ty == Ordering::Equal {
                a.layout.cmp(&b.layout)
            } else {
                cmp_ty
            }
        });
        println!("bytes reduced {:?}", dup_string_size);
        ConstantPool {
            string_pool,
            constants,
            layouts,
            types,
        }
    }
}

impl Read for ConstantPool {
    fn read(from: &mut dyn bytes::Buf) -> Self
    where
        Self: Sized,
    {
        Self {
            string_pool: read_vec(from, &mut |f| f.get_u8()),
            constants: read_vec(from, &mut |f| Constant::read(f)),
            layouts: read_vec(from, &mut |f| Layout::read(f)),
            types: read_vec(from, &mut |f| Type::read(f)),
        }
    }
}

impl Write for ConstantPool {
    fn write(self, to: &mut dyn bytes::BufMut) {
        write_vec(&self.string_pool, to, &mut |f, t| t.put_u8(f));
        write_vec(&self.constants, to, &mut |f, t| f.write(t));
        write_vec(&self.layouts, to, &mut |f, t| f.write(t));
        write_vec(&self.types, to, &mut |f, t| f.write(t));
    }
}

pub trait StaticSection:
    serde_traitobject::Serialize + serde_traitobject::Deserialize + Debug + Read + Write
{
}

pub trait ExecutableSection:
    serde_traitobject::Serialize + serde_traitobject::Deserialize + Debug + Read + Write
{
}

pub trait Info:
    serde_traitobject::Serialize + serde_traitobject::Deserialize + Debug + Read + Write
{
}
