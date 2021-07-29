use std::collections::HashSet;

use crate::{
    read_write::{read_vec, write_vec, Read, Write},
    BuildedLayout, LayoutBuilder,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Type {
    pub id: u32,
    /// match name in constant 0xFFFFFFFF means anonymous
    pub name: u32,
    /// match with constant Id in constant pool
    pub possible_layouts: Vec<u32>,
}

impl Read for Type {
    fn read(from: &mut dyn bytes::Buf) -> Self
    where
        Self: Sized,
    {
        Self {
            id: from.get_u32(),
            name: from.get_u32(),
            possible_layouts: read_vec(from, &mut |f| f.get_u32()),
        }
    }
}
impl Write for Type {
    fn write(self, to: &mut dyn bytes::BufMut) {
        to.put_u32(self.id);
        to.put_u32(self.name);
        write_vec(&self.possible_layouts, to, &mut |f, to| to.put_u32(f))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TypeBuilder {
    pub name: Option<String>,
    pub layouts: Vec<BuildedLayout>,
}

impl TypeBuilder {
    pub fn new() -> Self {
        Self {
            name: None,
            layouts: vec![],
        }
    }
    pub fn set_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
    pub fn add_layout(mut self, layout: BuildedLayout) -> Self {
        self.layouts.push(layout);
        self
    }
    pub fn build(self) -> Self {
        self
    }
}
