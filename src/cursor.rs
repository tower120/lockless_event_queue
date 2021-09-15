use std::cmp::Ordering;
use crate::event_queue::chunk::Chunk;

// TODO: Untested comparison!!
pub(crate) struct Cursor<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool>
{
    // TODO: try hide
    /// Always valid
    pub chunk: *const Chunk<T, CHUNK_SIZE>,
    /// in-chunk index
    pub index : usize
}

impl<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool>Copy for Cursor<T, CHUNK_SIZE, AUTO_CLEANUP> {}
impl<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool>Clone for Cursor<T, CHUNK_SIZE, AUTO_CLEANUP> {
    fn clone(&self) -> Self {
        Self{ chunk: self.chunk, index: self.index }
    }
}


impl<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool> Cursor<T, CHUNK_SIZE, AUTO_CLEANUP> {
    fn chunk_ref(&self) -> &Chunk<T, CHUNK_SIZE>{
        unsafe { &*self.chunk }
    }
}


impl<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool> PartialEq for Cursor<T, CHUNK_SIZE, AUTO_CLEANUP> {
    fn eq(&self, other: &Self) -> bool {
        self.chunk == other.chunk
        && self.index == other.index
    }
}
impl<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool> Eq for Cursor<T, CHUNK_SIZE, AUTO_CLEANUP>{}


impl<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool> PartialOrd for Cursor<T, CHUNK_SIZE, AUTO_CLEANUP> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }

    // TODO: Is this needed? Benchmark with/without specialized lt comparison
    fn lt(&self, other: &Self) -> bool {
        let self_chunk_id  = self.chunk_ref().id;
        let other_chunk_id = other.chunk_ref().id;

        if self_chunk_id < other_chunk_id{
            return true;
        }
        if self_chunk_id > other_chunk_id{
            return false;
        }
        return self.index < other.index;
    }
}
impl<T, const CHUNK_SIZE: usize, const AUTO_CLEANUP: bool> Ord for Cursor<T, CHUNK_SIZE, AUTO_CLEANUP> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_chunk_id  = self.chunk_ref().id;
        let other_chunk_id = other.chunk_ref().id;

        if self_chunk_id < other_chunk_id {
            return Ordering::Less;
        }
        if self_chunk_id > other_chunk_id {
            return Ordering::Greater;
        }
        self.index.cmp(&other.index)
    }
}