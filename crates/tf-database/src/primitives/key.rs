use crate::Result;

pub fn next_byte_sequence(start: &[u8]) -> Option<Vec<u8>> {
    let mut end = start.to_vec();
    // Modify the last byte by adding one. If it would wrap, we proceed to the
    // next byte.
    while let Some(last_byte) = end.pop() {
        if let Some(next) = last_byte.checked_add(1) {
            end.push(next);
            return Some(end);
        }
    }

    None
}

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
