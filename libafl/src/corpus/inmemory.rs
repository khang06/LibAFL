//! In-memory corpus, keeps all test cases in memory at all times

use alloc::vec::Vec;
use core::cell::RefCell;
use serde::{Deserialize, Serialize};

use crate::{corpus::Corpus, corpus::Testcase, inputs::Input, Error};

/// A corpus handling all in memory.
#[derive(Default, Serialize, Deserialize, Clone, Debug)]
#[serde(bound = "I: serde::de::DeserializeOwned")]
pub struct InMemoryCorpus<I>
where
    I: Input,
{
    entries: Vec<RefCell<Testcase<I>>>,
    current: Option<usize>,
}

impl<I> Corpus<I> for InMemoryCorpus<I>
where
    I: Input,
{
    /// Returns the number of elements
    #[inline]
    fn count(&self) -> usize {
        self.entries.len()
    }

    /// Add an entry to the corpus and return its index
    #[inline]
    fn add(&mut self, testcase: Testcase<I>) -> Result<usize, Error> {
        self.entries.push(RefCell::new(testcase));
        Ok(self.entries.len() - 1)
    }

    /// Replaces the testcase at the given idx
    #[inline]
    fn replace(&mut self, idx: usize, testcase: Testcase<I>) -> Result<(), Error> {
        if idx >= self.entries.len() {
            return Err(Error::KeyNotFound(format!("Index {} out of bounds", idx)));
        }
        self.entries[idx] = RefCell::new(testcase);
        Ok(())
    }

    /// Removes an entry from the corpus, returning it if it was present.
    #[inline]
    fn remove(&mut self, idx: usize) -> Result<Option<Testcase<I>>, Error> {
        if idx >= self.entries.len() {
            Ok(None)
        } else {
            Ok(Some(self.entries.remove(idx).into_inner()))
        }
    }

    /// Get by id
    #[inline]
    fn get(&self, idx: usize) -> Result<&RefCell<Testcase<I>>, Error> {
        Ok(&self.entries[idx])
    }

    /// Current testcase scheduled
    #[inline]
    fn current(&self) -> &Option<usize> {
        &self.current
    }

    /// Current testcase scheduled (mut)
    #[inline]
    fn current_mut(&mut self) -> &mut Option<usize> {
        &mut self.current
    }
}

impl<I> InMemoryCorpus<I>
where
    I: Input,
{
    pub fn new() -> Self {
        Self {
            entries: vec![],
            current: None,
        }
    }
}
