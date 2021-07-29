use crate::read_write::{Read, Write, read_vec, write_vec};
use serde::{Deserialize, Serialize};
use std::{collections::{HashMap, HashSet}, fmt::Debug, ptr::read};

#[cfg(test)]
mod test;

pub mod constant;
pub mod layout;
pub mod ty;
pub use constant::*;
pub use layout::*;
pub use ty::*;

#[derive(Debug, Clone, Serialize, Deserialize,PartialEq, Eq)]
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
        ty: TypeBuilder,
        layout: BuildedLayout,
        constant: Vec<u8>,
    ) -> Self {
        match self.types.get(&ty) {
            Some(_) => {
                self.constants.insert((ty, layout, constant));
            }
            None => {
                self.types.insert(ty.clone());
                self.constants.insert((ty, layout, constant));
            }
        };
        self
    }
    pub fn build(self) -> ConstantPool {
        let mut dublicated_symbols = 0;
        let mut symbol_counter = 0;
        let mut symbols = HashSet::new();
        let mut symbols_with_ids: HashMap<String, u32> = HashMap::new();

        let mut layouts = vec![];
        let mut types = vec![];
        let mut constants = vec![];
        let mut string_pool = vec![];
        let mut builded_layout_ids = HashMap::new();
        let mut type_builder_type_with_ids = HashMap::new();

        let mut add_symbol = |name: Option<String>| match name {
            Some(name) => {
                if symbols.insert(name.clone()) {
                    symbols_with_ids.insert(name, symbol_counter);
                    symbol_counter += 1;
                } else {
                    dublicated_symbols += 1;
                }
                symbol_counter - 1
            }
            None => 0xFFFFFFFF,
        };

        for ty in self.types {
            let name = add_symbol(ty.name.clone());
            let start_layouts_len = layouts.len() as u32;
            for BuildedLayout {
                length,
                field_names,
                field_length,
            } in &ty.layouts
            {
                let mut field_ids = Vec::with_capacity(field_length.len());
                for field_name in field_names {
                    field_ids.push(add_symbol(field_name.clone()));
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
        for mut cont in self.constants {
            let ty = *type_builder_type_with_ids.get(&cont.0).unwrap();
            let data_offset = string_pool.len() as u64;
            let layout = *builded_layout_ids.get(&cont.1).unwrap() as u32;
            string_pool.append(&mut cont.2);
            let constant = Constant {
                ty,
                layout,
                data_offset,
            };
            constants.push(constant);
        }
        ConstantPool {
            string_pool,
            constants,
            layouts,
            types,
        }
    }
}

impl Read for ConstantPool{
    fn read(from: &mut dyn bytes::Buf) -> Self
    where
        Self: Sized {
        Self{
            string_pool: read_vec(from, &mut |f| f.get_u8()),
            constants: read_vec(from,&mut |f| Constant::read(f)),
            layouts: read_vec(from, &mut |f| Layout::read(f)),
            types: read_vec(from,&mut |f| Type::read(f))
        }
    }
}

impl Write for ConstantPool{
    fn write(self, to: &mut dyn bytes::BufMut) {
        write_vec(&self.string_pool, to, &mut |f,t| t.put_u8(f));
        write_vec(&self.constants,to,&mut |f,t| f.write(t));
        write_vec(&self.layouts,to,&mut |f,t| f.write(t));
        write_vec(&self.types,to,&mut |f,t| f.write(t));
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
