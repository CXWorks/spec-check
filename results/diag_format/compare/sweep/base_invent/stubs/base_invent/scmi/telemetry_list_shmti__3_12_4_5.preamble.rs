use vstd::prelude::*;

verus! {

pub type int32 = i32;

pub type uint32 = u32;

pub struct ShmtiDesc {
    pub entry: Seq<uint32>,
}

pub type ShmtiDescList = Seq<ShmtiDesc>;

pub struct S {
    pub shmti_list: Seq<ShmtiDesc>,
    pub num_shmti: uint32,
}

pub const TELEMETRY_SUCCESS: int32 = 0;
pub const TELEMETRY_ERROR: int32 = 1;

} // verus!
