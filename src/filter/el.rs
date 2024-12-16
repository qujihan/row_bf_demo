#![allow(dead_code)]
use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use super::bitmap::BitMap;

// magic number
const EL_MAGIC: u32 = 0x05882967;

// EL_MAGIC + piece_size + bucket_index_bits
const EL_FILTER_HEADER: u32 = 4 + 4 + 4;

pub struct ELFilter<T: ?Sized> {
    bitmap: BitMap,
    piece_size: u32, // bytes
    bucket_index_bits: u32,
    _phantom: PhantomData<T>,
}

impl<T: ?Sized> ELFilter<T> {
    pub fn new_with_x(bucket_index_bits: u32, piece_size: u32) -> Self {
        let bitmap_size = EL_FILTER_HEADER + 2u32.pow(bucket_index_bits) + piece_size;
        let bitmap = BitMap::new(bitmap_size as usize);

        ELFilter {
            bitmap: bitmap,
            piece_size: piece_size,
            bucket_index_bits: bucket_index_bits,
            _phantom: PhantomData,
        }
    }

    fn filter_hash(&self, item: &T) -> u64
    where
        T: Hash,
    {
        0
    }

    fn bucket_start_offset(&self, hash: &u64) -> u64 {
        hash >> (64 - self.bucket_index_bits)
    }

    fn set_offsets_in_bucket(&self, hash: &u64) -> Vec<u64> {
        let mask = (1 << self.bucket_index_bits) - 1;
        let offsets_hash = hash & mask;
        let mut offsets = Vec::new();
        offsets.resize(4, 0);
        // for (i, offset) in offsets.iter_mut().enumerate() {

        // }
        offsets
    }

    pub fn set(&mut self, item: &T)
    where
        T: Hash,
    {
        let hash = self.filter_hash(item);
        let bucket_start_offet = self.bucket_start_offset(&hash);
        let set_offsets_in_bucket = self.set_offsets_in_bucket(&hash);
        for offset in set_offsets_in_bucket {
            self.bitmap.set((bucket_start_offet + offset) as usize);
        }
    }

    pub fn check(&self, item: &T) -> bool
    where
        T: Hash,
    {
        let hash = self.filter_hash(item);
        let bucket_start_offet = self.bucket_start_offset(&hash);
        let set_offsets_in_bucket = self.set_offsets_in_bucket(&hash);
        for offset in set_offsets_in_bucket {
            if !self.bitmap.get((bucket_start_offet + offset) as usize) {
                return false;
            }
        }
        true
    }
}
