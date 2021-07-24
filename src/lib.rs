pub mod binary;
pub mod checksum;
pub mod compression;
pub mod library;
pub mod vm;

pub use binary::*;
pub use checksum::*;
pub use compression::*;
pub use library::*;
pub use vm::*;

pub struct CVMB {
    // file meta info
    /// should be [0x43, 0x56, 0x4D, 0x42, 0x0A]
    pub magic_number: [u8; 5],
    pub checksum: Option<CheckSum>,
    pub compression: Option<CompressionInfo>,
    // VM related
    pub vm_specifier: Box<dyn VMSpecifier>,
    pub vm_info: Box<dyn VMInfo>,
    // lib related
    pub library_version: LibraryVersion,
    pub library_ext_info: Box<dyn LibraryEXTInfo>,
    pub access: Box<dyn Access>,
    pub dependencies: Option<Box<dyn Dependencies>>,
    // binaries
    pub info: Option<Box<dyn Info>>,
    pub constant_pool: ConstantPool,
    pub static_section: Box<dyn StaticSection>,
    pub executable_section: Box<dyn ExecutableSection>,
}
