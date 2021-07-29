use crate::read_write::{Read, Write};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Constant {
    pub ty: u32,
    pub layout: u32,
    pub data_offset: u64,
}

impl Read for Constant {
    fn read(from: &mut dyn bytes::Buf) -> Self {
        Constant {
            ty: from.get_u32(),
            layout: from.get_u32(),
            data_offset: from.get_u64(),
        }
    }
}

impl Write for Constant {
    fn write(self, to: &mut dyn bytes::BufMut) {
        to.put_u32(self.ty);
        to.put_u32(self.layout);
        to.put_u64(self.data_offset);
    }
}
