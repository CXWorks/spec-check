use vstd::prelude::*;

verus! {

pub type UInt64 = u64;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
    RmiErrorDevice,
}

pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> Result<T, E> {
    pub open spec fn is_Ok(&self) -> bool {
        match self {
            Result::Ok(_) => true,
            Result::Err(_) => false,
        }
    }

    pub open spec fn is_Err(&self) -> bool {
        match self {
            Result::Ok(_) => false,
            Result::Err(_) => true,
        }
    }
}

pub type DeviceClass = UInt64;

pub const NONSECURE_DEVICES: DeviceClass = 0;
pub const SECURE_DEVICES: DeviceClass = 1;

pub struct S {
    pub dummy: UInt64,
}

pub open spec fn DynamicLaunchRequestProxiedToCoprocessorDcrtm() -> bool;

pub open spec fn CallerDmaProtectionsBlock(devices: DeviceClass) -> bool;

} // verus!
