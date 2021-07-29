pub use bytes::*;

#[cfg(test)]
mod test;

pub trait Read {
    fn read(from: &mut dyn Buf) -> Self
    where
        Self: Sized;
}
pub trait Write {
    fn write(self, to: &mut dyn BufMut);
}

/// vec start with 32 bit len
pub fn read_vec<T>(from: &mut dyn Buf, read_t: &mut dyn FnMut(&mut dyn Buf) -> T) -> Vec<T> {
    let mut res = vec![];
    for _ in 0..from.get_u32() {
        res.push(read_t(from))
    }
    res
}
/// vec start with 32 bit len
pub fn write_vec<T: Clone>(
    from: &Vec<T>,
    to: &mut dyn BufMut,
    write_t: &mut dyn FnMut(T, &mut dyn BufMut),
) {
    to.put_u32(from.len() as u32);
    for i in 0..from.len() {
        write_t(from[i].clone(), to)
    }
}

/// option None == 0x00 Some == 0xFF
pub fn read_option<T>(from: &mut dyn Buf, read_t: &mut dyn FnMut(&mut dyn Buf) -> T) -> Option<T> {
    let mut res = None;
    if from.get_u8() != 0x00 {
        res = Some(read_t(from));
    }
    res
}
/// option None == 0x00 Some == 0xFF
pub fn write_option<T: Clone>(
    from: &Option<T>,
    to: &mut dyn BufMut,
    write_t: &mut dyn FnMut(T, &mut dyn BufMut),
) {
    match from {
        Some(f) => {
            to.put_u8(0xFF);
            write_t(f.clone(), to)
        }
        None => to.put_u8(0x00),
    }
}
