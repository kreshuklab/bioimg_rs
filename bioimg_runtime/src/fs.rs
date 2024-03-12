pub struct FsPath{
    components: Vec<String>
}

pub enum FileSystem{
    Local,
    Zip,
}

impl FileSystem{
    pub fn transfer_to(&self, target: &mut FileSystem){

    }
}