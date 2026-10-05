use vstd::prelude::*;

verus! {

// No preamble change can fix this error, so the preamble is returned unchanged.
// The problem is in the function itself: Verus needs a trigger for the forall
// over i. Each candidate term uses i inside arithmetic (base_attribute_id + i
// and 4 * i), and Verus does not allow arithmetic in triggers. The comparison
// i < attribute_count(old_s) cannot be a trigger either. Changing these
// declarations cannot fix that. The function needs a #[trigger] annotation or
// a #![auto] attribute.

pub type UInt32 = u32;
pub type UInt64 = u64;

pub enum sbiret {
    OK,
    ERR,
}

pub struct S {
    pub channel_id: UInt32,
    pub base_attribute_id: UInt32,
    pub calling_hart: UInt64,
}

pub open spec fn attribute_count(s: S) -> UInt32;

pub open spec fn ChannelAttribute(s: S, channel_id: UInt32, attribute_id: int) -> UInt32;

pub open spec fn SharedMemoryWord(hart: UInt64, offset: int) -> UInt32;

} // verus!
