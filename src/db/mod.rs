pub mod filename;
pub mod format;
pub mod iterator;


use std::fmt::Write;
use std::sync::Arc;
use crate::iterator::Iterator;
use crate::error::{Result, Error};
use crate::options::{WriteOptions, ReadOptions};
use crate::batch::WriteBatch;
use crate::snapshot::Snapshot;

pub trait DB {
    type Iterator;

    fn put(&self, write_opt: WriteOptions, key: &[u8], value: &[u8]) -> Result<()>;

    fn get(&self, read_opt: ReadOptions, key: &[u8]) -> Result<Option<Vec<u8>>>;

    fn iter(&self, read_opt: ReadOptions) -> Result<Self::Iterator>;

    fn delete(&self, write_opt: WriteOptions, key: &[u8]) -> Result<()>;

    fn write(&self, write_opt: WriteOptions, batch: WriteBatch) -> Result<()>;

    fn close(&mut self) -> Result<()>;

    fn destory(&mut self) -> Result<()>;

    fn snapshot(&self) -> Arc<Snapshot>;
}


#[derive!(Clone)]
pub struct MicroDB<S: Storage + Clone + 'static, C: Comparator> {
    // inner: Arc<>
}