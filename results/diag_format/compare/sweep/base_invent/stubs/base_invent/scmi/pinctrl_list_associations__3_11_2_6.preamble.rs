use vstd::prelude::*;

verus! {

// The spec uses NOT_FOUND and DENIED both as bool conditions (`NOT_FOUND ==> ...`)
// and as values compared with `result`. The only consistent typing is int32 = bool,
// so the four return codes cannot all be pairwise distinct.
// The compile error is an unclosed delimiter: the function's own text is cut off
// partway through its nested conditions. No declaration can fix that, so this
// preamble is unchanged.
pub type int32 = bool;
pub type uint32 = u32;
pub type uint16 = u16;

pub struct array<T> {
    pub len: u32,
    pub data: Seq<T>,
}

impl<T> array<T> {
    pub open spec fn spec_index(self, i: int) -> T {
        self.data[i]
    }
}

pub struct S {
    pub dummy: int,
}

pub const SUCCESS: int32 = true;
pub const NOT_FOUND: int32 = false;
pub const NOT_SUPPORTED: int32 = true;
pub const DENIED: int32 = false;

} // verus!
