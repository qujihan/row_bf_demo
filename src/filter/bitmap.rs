#![allow(unused)]
use std::convert::{TryFrom, TryInto};
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct BitMap {
    bits: Vec<u8>,
}

impl BitMap {
    pub fn new(len_bytes: usize) -> Self {
        Self {
            bits: vec![0; len_bytes],
        }
    }

    #[inline]
    fn bits(&self) -> &[u8] {
        &self.bits[..]
    }

    #[inline]
    fn bits_mut(&mut self) -> &mut [u8] {
        &mut self.bits[..]
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.bits
    }

    #[inline]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bits
    }

    #[inline]
    pub fn to_bytes(&self) -> Vec<u8> {
        self.bits.clone()
    }

    pub fn get(&self, bit_offset: usize) -> bool {
        let byte_offset = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        (self.bits()[byte_offset] & (1 << bit_shift)) != 0
    }

    pub fn set(&mut self, bit_offset: usize) {
        let byte_offset = bit_offset / 8;
        let bit_shift = bit_offset % 8;
        self.bits_mut()[byte_offset] |= 1 << bit_shift;
    }

    pub fn clear(&mut self) {
        for byte in self.bits_mut().iter_mut() {
            *byte = 0;
        }
    }

    pub fn set_all(&mut self) {
        for byte in self.bits_mut().iter_mut() {
            *byte = !0;
        }
    }
}
