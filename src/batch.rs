pub const HEADER_SIZE: usize = 12;

#[derive(Clone)]
pub struct WriteBatch {
    contents: Vec<u8>,
}

impl Default for WriteBatch {
    fn default() -> Self {
        let contents = vec![0; HEADER_SIZE];
        Self { contents }
    }
}

impl WriteBatch {
    #[inline]
    pub fn data(&self) -> &[u8] {
        self.contents.as_slice()
    }
}
