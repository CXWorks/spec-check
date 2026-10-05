use vstd::prelude::*;

verus! {

pub type UInt32 = u32;
pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

pub struct S {
    pub cppc_regs: Map<UInt32, UInt64>,
}

pub spec const SBI_SUCCESS: sbiret = sbiret { error: 0i64, value: 0u64 };
pub spec const SBI_ERR_NOT_SUPPORTED: sbiret = sbiret { error: -2i64, value: 0u64 };
pub spec const SBI_ERR_INVALID_PARAM: sbiret = sbiret { error: -3i64, value: 0u64 };

pub open spec fn IsReservedCppcReg(s: S, cppc_reg_id: UInt32) -> bool;

pub open spec fn IsImplementedCppcReg(s: S, cppc_reg_id: UInt32) -> bool;

pub open spec fn ResultEqual(result: sbiret, expected: sbiret) -> bool;

pub open spec fn CppcReg(s: S, cppc_reg_id: UInt32) -> UInt64;

} // verus!
