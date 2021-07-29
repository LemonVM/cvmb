use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

#[cfg(test)]
mod test;

#[derive(PartialEq, Eq, Debug, Default, Clone, Copy, Serialize, Deserialize)]
pub struct LibraryVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl PartialOrd for LibraryVersion {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let major_cmp = self.major.cmp(&other.major);
        if major_cmp != Ordering::Equal {
            return Some(major_cmp);
        } else {
            let minor_cmp = self.minor.cmp(&other.minor);
            if minor_cmp != Ordering::Equal {
                return Some(minor_cmp);
            } else {
                let patch_cmp = self.patch.cmp(&other.patch);
                if patch_cmp != Ordering::Equal {
                    return Some(patch_cmp);
                } else {
                    return Some(Ordering::Equal);
                }
            }
        }
    }
}

impl Ord for LibraryVersion {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub trait LibraryEXTInfo: serde_traitobject::Serialize + serde_traitobject::Deserialize {}
pub trait Access: serde_traitobject::Serialize + serde_traitobject::Deserialize {}
pub trait Dependencies: serde_traitobject::Serialize + serde_traitobject::Deserialize {}
