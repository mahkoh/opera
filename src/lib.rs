#![cfg_attr(not(test), no_std)]
//! This crate provides marker types that are easier to understand than using
//! [PhantomData] directly.

#[cfg(test)]
mod tests;

use core::marker::PhantomData;

/// A marker type that implements [`!Send`](Send).
///
/// Construct it using [Default::default()].
#[derive(Default)]
pub struct PhantomNotSend {
    marker: PhantomData<*mut u8>,
}

unsafe impl Sync for PhantomNotSend {}

/// A marker type that implements [`!Sync`](Sync).
///
/// Construct it using [Default::default()].
#[derive(Default)]
pub struct PhantomNotSync {
    marker: PhantomData<*mut u8>,
}

unsafe impl Send for PhantomNotSync {}
