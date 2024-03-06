use std::path::PathBuf;

pub struct PackageComponent<'a, T: serde::Serialize> {
    pub rdf_value: T,
    pub binary: (PathBuf, &'a [u8]), //FIXME: PatHBuf might be incompatible with Zip paths
}
