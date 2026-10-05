use vstd::prelude::*;

verus! {

pub type MigrateInfoType = i32;

pub type PsciReturn = i32;

pub type PsciCall = u64;

pub type CpuId = u64;

pub struct S {
    pub trusted_os_present: bool,
    pub trusted_os_uniprocessor: bool,
    pub trusted_os_migrate_capable: bool,
    pub trusted_os_mp_aware: bool,
    pub trusted_os_resident_cpu: CpuId,
    pub previous_migrate_info_type_result: MigrateInfoType,
}

pub const SUCCESS: PsciReturn = 0;

pub const NOT_SUPPORTED: PsciReturn = -1;

pub const INVALID_PARAMETERS: PsciReturn = -2;

pub const DENIED: PsciReturn = -3;

pub const CPU_OFF: PsciCall = 0x8400_0002;

pub const target_cpu: CpuId = 1;

pub open spec fn TrustedOsIsUniprocessor(s: S) -> bool;

pub open spec fn TrustedOsIsMigrateCapable(s: S) -> bool;

pub open spec fn TrustedOsIsMultiprocessorAware(s: S) -> bool;

pub open spec fn TrustedOsIsPresent(s: S) -> bool;

pub open spec fn PreviousMigrateInfoTypeResult(s: S) -> MigrateInfoType;

pub open spec fn TrustedOsResidentCpu(s: S) -> CpuId;

pub open spec fn IsValidMpidr(s: S, cpu: CpuId) -> bool;

pub open spec fn MIGRATE(cpu: CpuId) -> PsciCall;

pub open spec fn ReturnOf(call: PsciCall, cpu: CpuId) -> PsciReturn;

} // verus!
