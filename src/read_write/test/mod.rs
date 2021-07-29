use bytes::{Buf, BufMut, Bytes};

use crate::read_write::write_vec;

use super::{read_vec, Read, Write};

struct Gay {
    name: u32,
}

impl Read for Gay {
    fn read(from: &mut dyn Buf) -> Self {
        Gay {
            name: from.get_u32(),
        }
    }
}
impl Write for Gay {
    fn write(self, to: &mut dyn BufMut) {
        to.put_u32(self.name)
    }
}

#[test]
fn test_read() {
    let mut read_from = Bytes::from(&([0x00, 0x00, 0xFF, 0xFF] as [u8; 4])[..]);
    let gay = Gay::read(&mut read_from);
    assert_eq!(gay.name, 0xFFFF)
}

static TEST_VEC: [u8; 6] = [0x00, 0x00, 0x00, 0x02, 0x01, 0x02];
#[test]
fn test_read_vec() {
    let res = read_vec(&mut Bytes::from(&TEST_VEC[..]), &mut |f| f.get_u8());
    assert_eq!(res, vec![1, 2]);
}

#[test]
fn test_write_vec() {
    let mut to = vec![];
    let from = vec![1, 2];
    write_vec(&from, &mut to, &mut |f, t| t.put_u8(f));
    assert_eq!(to, Vec::from(TEST_VEC));
}
