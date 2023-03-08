/// Options that control read operations
#[derive(Clone, Copy)]
pub struct ReadOptions {
    /// If true, all data read from underlying storage will be
    /// verified against corresponding checksums.
    pub verify_checksums: bool,

    /// Should the data read for this iteration be cached in memory?
    /// Callers may wish to set this field to false for bulk scans.
    pub fill_cache: bool,

    /// If `snapshot` is `None`, read as of the supplied snapshot
    /// (which must belong to the DB that is being read and which must
    /// not have been released).  If `snapshot` is `None`, use an implicit
    /// snapshot of the state at the beginning of this read operation.
    pub snapshot: Option<Snapshot>,
}

impl Default for ReadOptions {
    fn default() -> Self {
        ReadOptions {
            verify_checksums: false,
            fill_cache: true,
            snapshot: None,
        }
    }
}


#[derive(Default)]
pub struct WriteOptions {
    /// If true, the write will be flushed from the operating system
    /// buffer cache before the write is considered complete.
    /// If this flag is true, writes will be slower.
    ///
    /// If this flag is false, and the machine crashes, some recent
    /// writes may be lost.  Note that if it is just the process that
    /// crashes (i.e., the machine does not reboot), no writes will be
    /// lost even if sync==false.
    ///
    /// In other words, a DB write with sync==false has similar
    /// crash semantics as the "write()" system call.  A DB write
    /// with sync==true has similar crash semantics to a "write()"
    /// system call followed by "fsync()".
    pub sync: bool,
}

