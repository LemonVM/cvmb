pub mod checksum;
pub mod compression;
pub mod constant_pool;
pub mod library;
pub mod vm;

pub mod read_write;

pub use checksum::*;
pub use compression::*;
pub use constant_pool::*;
pub use library::*;
pub use vm::*;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CVMB {
    // file meta info
    /// should be [0x43, 0x56, 0x4D, 0x42, 0x0A]
    pub magic_number: [u8; 5],
    pub checksum: Option<CheckSum>,
    pub compression: Option<CompressionInfo>,

    // VM related
    #[serde(with = "serde_traitobject")]
    pub vm_specifier: Box<dyn VMSpecifier>,
    #[serde(with = "serde_traitobject")]
    pub vm_info: Box<dyn VMInfo>,

    // lib related
    pub library_version: LibraryVersion,
    #[serde(with = "serde_traitobject")]
    pub library_ext_info: Box<dyn LibraryEXTInfo>,
    #[serde(with = "serde_traitobject")]
    pub access: Box<dyn Access>,
    #[serde(with = "serde_traitobject")]
    pub dependencies: Box<dyn Dependencies>,

    // binaries
    #[serde(with = "serde_traitobject")]
    pub info: Box<dyn Info>,
    pub constant_pool: ConstantPool,
    #[serde(with = "serde_traitobject")]
    pub static_section: Box<dyn StaticSection>,
    #[serde(with = "serde_traitobject")]
    pub executable_section: Box<dyn ExecutableSection>,
}
