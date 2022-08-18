use crate::Result;

pub trait Key
where
    Self: Sized,
{
    fn as_key(&self) -> Vec<u8>;
    fn as_prefix(&self) -> [u8; 21];
    fn from_bytes(bytes: &[u8]) -> Result<Self>;
    fn as_string(&self) -> String {
        String::from_utf8(self.as_key()).unwrap()
    }
}

impl Key for String {
    fn as_key(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    fn as_prefix(&self) -> [u8; 21] {
        [0; 21]
    }

    fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(String::from_utf8(bytes.to_vec()).unwrap())
    }
}
