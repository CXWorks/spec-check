use vstd::prelude::*;
verus! {

pub type DeviceClass = u64;

pub const NONSECURE_DEVICES: DeviceClass = 0;
pub const SECURE_DEVICES: DeviceClass = 1;

pub enum RmiStatusCode {
    RmiSuccess,
    RmiErrorInput,
    RmiErrorRealm,
    RmiErrorRec,
    RmiErrorRtt,
    RmiErrorDevice,
}

pub struct S {
    pub dcrtm_launch_requested: bool,
    pub nonsecure_dma_blocked: bool,
    pub secure_dma_blocked: bool,
}

pub open spec fn DynamicLaunchRequestProxiedToCoprocessorDcrtm(s: S) -> bool;

pub open spec fn CallerDmaProtectionsBlock(s: S, devices: DeviceClass) -> bool;

} // verus!
