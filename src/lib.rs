//! Concurrent FIFO event queue / message queue. Multi consumer. Each consumer receive all messages.
//! Lock-free reading. Write under lock (for [mpmc] version). Write lock does not block read.
//!
//! Linked list of chunks (C++ std::deque -like). Each chunk have "read counter".
//! When "read counter" reach readers count - chunk dropped. Chunk considered read, when
//! Reader reach its end. See `doc/principal-of-operation.md`.
//!
//! `EventQueue` live, until `EventReader`s live.
//! In order to completely drop `EventQueue` - drop all associated `EventReader`s.
//!
//! # Features
//!
//! * `double_buffering` : Reuse biggest freed chunk.

mod sync;
mod utils;
mod cursor;
mod event_queue;
mod event_reader;
mod chunk_state;
#[allow(dead_code)]
mod dynamic_array;

// TODO: make double_buffering not a feature.
#[allow(dead_code)]
mod dynamic_chunk;

/// Epoch of EventQueue::start_position
type StartPositionEpoch = crate::utils::Epoch<u32, {i32::MAX as u64}>;

pub mod mpmc;

pub mod spmc{
    //! Single-producer multi-consumer.
    //! 
    //! Same as [mpmc](crate::mpmc), but writes without lock.
    //!
    //! CLEANUP happens on new chunk allocation. _Since there is no more lock - reader can not
    //! safely call cleanup_
    //!
    //! To be implemented.
}


#[cfg(test)]
mod tests;