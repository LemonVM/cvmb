#[cfg(test)]
mod test;

pub trait VMSpecifier: serde_traitobject::Serialize + serde_traitobject::Deserialize {}
pub trait VMInfo: serde_traitobject::Serialize + serde_traitobject::Deserialize {}
