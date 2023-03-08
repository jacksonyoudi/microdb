pub mod filename;
pub mod format;
pub mod iterator;


use crate::iterator::Iterator;
use crate::error::{Result, Error};
use crate::options::{WriteOptions, ReadOptions};

pub trait DB {
    type Iterator;

    fn put(&self, write_opt: WriteOptions, key: &[u8], value: &[u8]) -> Result<()>;

    fn get(&self, read_option: ReadOptions, key: &[u8]) -> Result<Option<Vec<u8>>>;
}