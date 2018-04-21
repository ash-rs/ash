use std::path::Path;
use std::io::Read;
use failure::Error;
use xml;
pub struct VendorId {
    pub name: String,
    pub id: String,
    pub comment: String,
}
impl VendorId {
    pub fn from_xml<P: AsRef<Path>>(path: P) -> Self {
        unimplemented!()
    }
}

pub struct VendorIds {
    pub vendor_ids: Vec<VendorId>,
}
impl VendorIds{
    pub fn from_xml<P: AsRef<Path>>(path: P) -> Self {
        unimplemented!()
    }
}

pub struct Registry  {
    pub vendor_ids: VendorIds,
}

impl Registry {
    pub fn from_xml<P: AsRef<Path>>(path: P) -> Result<Registry, Error> {
        fn inner(path: &Path) -> Result<Registry, Error> {
            use std::fs::File;
            use std::io::BufReader;
            let file = File::open(path)?;
            let buf_reader = BufReader::new(file);
            let event_reader = xml::EventReader::new(buf_reader);
            unimplemented!()
        }
        inner(path.as_ref())
    }
}
