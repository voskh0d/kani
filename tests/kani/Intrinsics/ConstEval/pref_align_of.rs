// Copyright Kani Contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT

// Check that we get the expected results for the `pref_align_of` intrinsic
// with common data types
#![feature(core_intrinsics)]
use std::intrinsics::pref_align_of;

struct MyStruct {}

enum MyEnum {}

#[kani::proof]
fn main() {
    // Scalar types
    assert!(unsafe { pref_align_of::<i8>() } == 1);
    assert!(unsafe { pref_align_of::<i16>() } == 2);
    assert!(unsafe { pref_align_of::<i32>() } == 4);
    assert!(unsafe { pref_align_of::<i64>() } == 8);
    assert!(unsafe { pref_align_of::<i128>() } == 8);
    assert!(unsafe { pref_align_of::<isize>() } == 8);
    assert!(unsafe { pref_align_of::<u8>() } == 1);
    assert!(unsafe { pref_align_of::<u16>() } == 2);
    assert!(unsafe { pref_align_of::<u32>() } == 4);
    assert!(unsafe { pref_align_of::<u64>() } == 8);
    assert!(unsafe { pref_align_of::<u128>() } == 8);
    assert!(unsafe { pref_align_of::<usize>() } == 8);
    assert!(unsafe { pref_align_of::<f32>() } == 4);
    assert!(unsafe { pref_align_of::<f64>() } == 8);
    assert!(unsafe { pref_align_of::<bool>() } == 1);
    assert!(unsafe { pref_align_of::<char>() } == 4);
    // Compound types (tuple and array)
    assert!(unsafe { pref_align_of::<(i32, i32)>() } == 8);
    assert!(unsafe { pref_align_of::<[i32; 5]>() } == 4);
    // Custom data types (struct and enum)
    assert!(unsafe { pref_align_of::<MyStruct>() } == 8);
    assert!(unsafe { pref_align_of::<MyEnum>() } == 1);
}
