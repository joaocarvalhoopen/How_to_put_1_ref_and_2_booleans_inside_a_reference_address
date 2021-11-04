// Name: Rust - How to put 1 ref and 2 booleans inside a reference address?
//
// Description: ref_with_2_flags for at least 4 bytes aligned types like i32,
//              u32, i64, u64, or other structure type of more than 4 bytes
//              in size. It gives you the ability of having a reference and
//              two flags in the same memory space of a reference (memory
//              address / pointer).
//
// Is this magic? 
// How does this work? : This uses intelligently the fact that memory address's
//              have to be aligned to the word, or to other size in memory.
//              So there are values that can't occur in the address of the 
//              reference or pointer.
//
//              Example:
//                 For a 2 byte alignment the possible address's are always even,
//                 they can't be odd.
//                 The values can be terminated in 0, or 2, or 4 ....
//                 0x00...000, 0x00...010, 0x00...100, 0x00...110,
//                 But can't be terminated in 1, or 3, or 5 .....
//                 0x00...001, 0x00...011, 0x00...101, 0x00...111,
//                 Do you see the pattern here?
//                 So you can use this extra bit value to put the state of your
//                 boolean flag in the some u64 address, in there unused values,
//                 or bit.  
//                 When you return the reference you return always the correct
//                 address step.
//
//              Example:
//                 For a 4 byte alignment the possible address are always aligned,
//                 in memory to 4 bytes.
//                 Do values can be terminated in 0, or 4, or 8 ....
//                 0x00...0000, 0x00...0100, 0x00...1000, 0x00...1100,
//                 But can't be terminated in 1, or 2, or 3, ....5, or 6, or 7...
//                 0x00...0001, 0x00...0010, 0x00...0011,
//                 or
//                 0x00...0101, 0x00...0110, 0x00...0111,
//                 Do you see the pattern here?
//                 So you can use this extra 2 bit values to put the state of your
//                 2 boolean flags in the some u64 address, in there unused values
//                 or bits.  
//                 When you return the reference you return always the correct
//                 address step.
//
// Date: 2021.11.04
//
// Author (derived work): João Nuno Carvalho
//
// Derived work from: This is a derived work from the example present on page 642
//                    on the fantastic Rust book:
//
//    Programming Rust: Fast, Safe Systems Development 2nd Edition
//    by Jim Blandy, Jason Orendorff, Leonora F. S. Tindall
//
//    I highly recommend this books!
// 
//    In this book, this example is called ref_with_flag and it allows to have a
//    boolean and a reference without using more space then the reference.
//    The example given, is for 2 byte or more alignment data types, it can't be
//    used for one byte data types like u8, i8 or bool. 
//    But can be used with everything else.
//    This can be extrapolated for any type of structure size with even more bytes
//    of alignment, imagine if you have a 20 bytes alignment!
//    What could you fit in this space?
//    The book says this is a very common technique used in implementations of
//    Garbage Collectors to save space in there references.
//    Because the default value of integer numbers in Rust is the 4 byte (32 bits)
//    i32, I extended here the example to a 4 bytes alignment with 2 flags instead
//    of one flag.
//    But the idea is the same if you want to extend to any size.
//    if you had objects that occupy 128 Bytes alignment, you could even put ASCII
//    characters inside them, without occupying any space in memory :-)
//    Nice! 
//
// Because this is a derived work the license is the same as the original code.                                 


use std::marker::PhantomData;
use std::mem::align_of;

pub  struct RefWith2Flags<'a, T> {
    ptr_and_bit: usize,
    behaves_like: PhantomData<&'a T> // occupies no space
}

impl<'a, T: 'a> RefWith2Flags<'a, T> {

    pub fn new(ptr: &'a T, flag_a: bool, flag_b: bool) -> RefWith2Flags<T> {
        assert!(align_of::<T>() % 4 == 0);
        RefWith2Flags {
            ptr_and_bit: ptr as *const T as usize | flag_a as usize | ((flag_b as usize) << 1),
            behaves_like: PhantomData
        }
    }

    pub fn get_ref(&self) -> &'a T {
        unsafe {
            let ptr = (self.ptr_and_bit & !3) as *const T;
            &*ptr
            }
    }
    
    pub fn get_flag_a(&self) -> bool {
        self.ptr_and_bit & 1 != 0
    }

    pub fn get_flag_b(&self) -> bool {
        self.ptr_and_bit & 2 != 0
    }

}

        



