use crate::{Error, Result};

pub trait Iterator {
    fn valid(&self) -> bool;

    fn seek_to_first(&mut self);

    fn seek_to_last(&mut self);

    fn seek(&mut self, target: &[u8]);

    fn next(&mut self);

    fn prev(&mut self);

    fn key(&mut self);

    fn value(&mut self);

    fn status(&mut self) -> Result<(), E>;
}