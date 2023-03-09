const MIN_SNAPSHOT: u64 = 0;

#[derive(Eq, PartialEq, Ord, PartialOrd, Clone, Copy, Debug)]
pub struct Snapshot {
    // The sequence number pointing to the view of db
    sequence_number: u64,
}

impl Snapshot {
    #[inline]
    pub fn sequence(self) -> u64 {
        self.sequence_number
    }
}