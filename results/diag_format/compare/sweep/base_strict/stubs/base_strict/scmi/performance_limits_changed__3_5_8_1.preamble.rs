use vstd::prelude::*;
verus! {

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
}

pub struct S {
    pub dummy: u64,
}

} // verus!
