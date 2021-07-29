use std::{collections::HashSet, vec};

use crate::read_write::{read_vec, write_vec, Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Layout {
    pub id: u32,
    /// 0xFF 0xFF means dynamic lengthed, 0x00 means a tag
    pub length: u16,
    /// 0x00 means dynamic sized field
    pub field_length: Vec<u8>,
    /// 0xFFFFFFFF means anonymous
    pub field_names: Vec<u32>,
}

pub struct LayoutBuilder {
    fields: HashSet<(Option<String>, u8)>,
}

impl LayoutBuilder {
    pub fn new() -> Self {
        Self {
            fields: HashSet::new(),
        }
    }
    pub fn add_field(mut self, name: Option<String>, length: u8) -> Self {
        self.fields.insert((name, length));
        self
    }
    pub fn build(self) -> BuildedLayout {
        let mut res = BuildedLayout {
            length: 0x0000,
            field_names: vec![],
            field_length: vec![],
        };
        for (name, length) in self.fields {
            res.field_names.push(name);
            res.field_length.push(length);
            if length == 0x00 {
                res.length = 0xFFFF;
            }
        }
        if res.length != 0xFFFF {
            res.length = res
                .field_length
                .iter()
                .fold(0 as u16, |acc, x| acc + *x as u16);
        }
        res
    }
}

impl Read for Layout {
    fn read(from: &mut dyn bytes::Buf) -> Self
    where
        Self: Sized,
    {
        Self {
            id: from.get_u32(),
            length: from.get_u16(),
            field_length: read_vec(from, &mut |r| r.get_u8()),
            field_names: read_vec(from, &mut |r| r.get_u32()),
        }
    }
}
impl Write for Layout {
    fn write(self, to: &mut dyn bytes::BufMut) {
        to.put_u32(self.id);
        to.put_u16(self.length);
        write_vec(&self.field_length, to, &mut |f, to| to.put_u8(f));
        write_vec(&self.field_names, to, &mut |f, to| to.put_u32(f));
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BuildedLayout {
    pub(crate) length: u16,
    pub(crate) field_names: Vec<Option<String>>,
    pub(crate) field_length: Vec<u8>,
}
