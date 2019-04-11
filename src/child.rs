use crate::key::BytesKey;
use crate::node::BytesNode;
use std::fmt;
use std::fmt::Pointer;

pub(crate) const MAX_CHILD_SIZE: usize = 256;

pub(crate) enum Child<K: BytesKey, T> {
    _1(Box<[Option<BytesNode<K, T>>; 1]>),
    _2(Box<[Option<BytesNode<K, T>>; 2]>),
    _4(Box<[Option<BytesNode<K, T>>; 4]>),
    _8(Box<[Option<BytesNode<K, T>>; 8]>),
    _16(Box<[Option<BytesNode<K, T>>; 16]>),
    _32(Box<[Option<BytesNode<K, T>>; 32]>),
    _64(Box<[Option<BytesNode<K, T>>; 64]>),
    _128(Box<[Option<BytesNode<K, T>>; 128]>),
    _256(Box<[Option<BytesNode<K, T>>; 256]>),
}

impl<K: BytesKey, T> Child<K, T> {
    pub(crate) fn new(size: usize) -> Self {
        match size {
            1 => Self::new_1(),
            2 => Self::new_2(),
            4 => Self::new_4(),
            8 => Self::new_8(),
            16 => Self::new_16(),
            32 => Self::new_32(),
            64 => Self::new_64(),
            128 => Self::new_128(),
            256 => Self::new_256(),
            _ => Self::new_1(),
        }
    }

    pub(crate) fn slot(&self, hash: u8) -> usize {
        hash as usize % self.size()
    }

    pub(crate) fn at(&mut self, slot: usize) -> Option<&mut BytesNode<K, T>> {
        self.child_mut()[slot].as_mut()
    }

    pub(crate) fn put(&mut self, slot: usize, node: BytesNode<K, T>) {
        let child = self.child_mut();
        child[slot] = Some(node);
    }

    pub(crate) fn size(&self) -> usize {
        match self {
            Child::_1(_) => 1,
            Child::_2(_) => 2,
            Child::_4(_) => 4,
            Child::_8(_) => 8,
            Child::_16(_) => 16,
            Child::_32(_) => 32,
            Child::_64(_) => 64,
            Child::_128(_) => 128,
            Child::_256(_) => 256,
        }
    }

    pub(crate) fn child_mut(&mut self) -> &mut [Option<BytesNode<K, T>>] {
        match self {
            Child::_1(c) => c.as_mut().as_mut(),
            Child::_2(c) => c.as_mut().as_mut(),
            Child::_4(c) => c.as_mut().as_mut(),
            Child::_8(c) => c.as_mut().as_mut(),
            Child::_16(c) => c.as_mut().as_mut(),
            Child::_32(c) => c.as_mut().as_mut(),
            Child::_64(c) => c.as_mut().as_mut(),
            Child::_128(c) => c.as_mut().as_mut(),
            Child::_256(c) => c.as_mut().as_mut(),
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn child(&self) -> &[Option<BytesNode<K, T>>] {
        match self {
            Child::_1(c) => c.as_ref().as_ref(),
            Child::_2(c) => c.as_ref().as_ref(),
            Child::_4(c) => c.as_ref().as_ref(),
            Child::_8(c) => c.as_ref().as_ref(),
            Child::_16(c) => c.as_ref().as_ref(),
            Child::_32(c) => c.as_ref().as_ref(),
            Child::_64(c) => c.as_ref().as_ref(),
            Child::_128(c) => c.as_ref().as_ref(),
            Child::_256(c) => c.as_ref().as_ref(),
        }
    }

    #[cfg(feature = "serde")]
    pub(crate) fn is_empty(&self) -> bool {
        self.child().iter().all(Option::is_none)
    }
}

// I'm not sure why this works but deriving `Debug` on the enum doesn't.
// It was complaining about the arrays larger than 32, but it seems like
// `fmt()` is just doing the same thing to the box?  My guess is that
// deriving it dereferences `Box` or something similar.
impl<K: BytesKey, T: fmt::Debug> fmt::Debug for Child<K, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Child::_1(c) => c.fmt(f),
            Child::_2(c) => c.fmt(f),
            Child::_4(c) => c.fmt(f),
            Child::_8(c) => c.fmt(f),
            Child::_16(c) => c.fmt(f),
            Child::_32(c) => c.fmt(f),
            Child::_64(c) => c.fmt(f),
            Child::_128(c) => c.fmt(f),
            Child::_256(c) => c.fmt(f),
        }
    }
}

// Since we want to create a lot of `None`s of a type that isn't `Copy` and
// we want our arrays larger than 32 (the maximum size for `Default`) we need
// to use literals.  I played with making a proc_macro to generate the literals
// but I couldn't figure out how to use the proc_macro in the macro_rules and
// since our type was generic the literal didn't get type inference unless used
// inside an impl.
//
// Oh well, it was just a one time copy and paste and just looks funny.
// I probably spent half an hour messing with it, and 2 minutes to copy paste.
macro_rules! child_new_init {
    ($new_fn:ident, $init:expr) => {
        impl<K: BytesKey, T> Child<K, T> {
            /// Create sized empty child
            pub(crate) fn $new_fn() -> Self {
                $init
            }
        }
    };
}

child_new_init!(new_1, Child::_1(Box::new([None])));
child_new_init!(new_2, Child::_2(Box::new([None, None])));
child_new_init!(new_4, Child::_4(Box::new([None, None, None, None])));
child_new_init!(
    new_8,
    Child::_8(Box::new([None, None, None, None, None, None, None, None]))
);
child_new_init!(
    new_16,
    Child::_16(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None
    ]))
);
child_new_init!(
    new_32,
    Child::_32(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None
    ]))
);
child_new_init!(
    new_64,
    Child::_64(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None
    ]))
);
child_new_init!(
    new_128,
    Child::_128(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None
    ]))
);
child_new_init!(
    new_256,
    Child::_256(Box::new([
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None, None, None, None, None, None, None, None, None, None, None, None, None, None, None,
        None
    ]))
);
