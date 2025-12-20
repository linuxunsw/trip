use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;

#[derive(Debug, Default)]
pub struct TileCache {
     path: PathBuf
}

impl TileCache {
    pub fn create() -> io::Result<Self> {
        /* build path */ 
        let mut cache_path = env::home_dir()
            .ok_or(io::Error::new(io::ErrorKind::NotFound, "no home dir"))?;
        cache_path.push(".cache/trip");
        fs::create_dir_all(&cache_path)?;

        Ok(Self{
            path: cache_path
        })
    }

    pub fn insert_key(&self, key: String, val: &[u8]) -> io::Result<()> {
        let path = self.path.join(key);
        fs::write(path, val)?;
        Ok(())
    }

    pub fn get_key(&self, key: String) -> io::Result<Vec<u8>> {
        let path = self.path.join(key);
        let val = fs::read(path)?;
        Ok(val)
    }
}
