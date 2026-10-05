use vstd::prelude::*;

verus! {

pub type MigrateInfoType = i32;

pub type PsciReturn = i32;

pub type PsciCall = u32;

pub type Cpu = u64;

pub enum Mpidr {
    Value(u64),
    UNDEFINED,
}

pub struct S {
    pub migrate_info_type: MigrateInfoType,
    pub resident_cpu: Cpu,
}

pub spec const SUCCESS: PsciReturn = 0;

pub spec const NOT_SUPPORTED: PsciReturn = -1;

pub spec const DENIED: PsciReturn = -3;

pub spec const CPU_OFF: PsciCall = 0x84000002;

pub spec const target_cpu: Mpidr = Mpidr::Value(0);

pub open spec fn TrustedOsIsUniprocessor() -> bool;

pub open spec fn TrustedOsIsMigrateCapable() -> bool;

pub open spec fn TrustedOsIsMultiprocessorAware() -> bool;

pub open spec fn TrustedOsIsPresent() -> bool;

pub open spec fn ResultEqual(result: MigrateInfoType, value: i32) -> bool;

pub open spec fn ReturnOf(call: PsciCall) -> PsciReturn;

pub open spec fn MIGRATE(target: Mpidr) -> PsciCall;

pub open spec fn TrustedOsResidentCpu() -> Cpu;

pub open spec fn IsValidMpidr(mpidr: Mpidr) -> bool;

pub open spec fn PreviousMigrateInfoTypeResult() -> MigrateInfoType;

pub open spec fn MigrateInfoType() -> MigrateInfoType;

pub open spec fn MpidrOf(cpu: Cpu) -> Mpidr;

} // verus!
