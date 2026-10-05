use vstd::prelude::*;
verus! {

pub type UInt64 = u64;

pub struct sbiret {
    pub error: i64,
    pub value: u64,
}

impl sbiret {
    pub open spec fn is_Ok(self) -> bool {
        self.error == 0
    }

    pub open spec fn is_Err(self) -> bool {
        self.error != 0
    }
}

pub struct S {
    pub cmd_input_counter_idx_base: u64,
    pub cmd_input_counter_idx_mask: u64,
    pub cmd_input_start_flags: u64,
    pub cmd_input_initial_value: u64,
}

pub spec const SBI_SUCCESS: i64 = 0;
pub spec const SBI_ERR_FAILED: i64 = -1;
pub spec const SBI_ERR_NOT_SUPPORTED: i64 = -2;
pub spec const SBI_ERR_INVALID_PARAM: i64 = -3;
pub spec const SBI_ERR_DENIED: i64 = -4;
pub spec const SBI_ERR_INVALID_ADDRESS: i64 = -5;
pub spec const SBI_ERR_ALREADY_AVAILABLE: i64 = -6;
pub spec const SBI_ERR_ALREADY_STARTED: i64 = -7;
pub spec const SBI_ERR_ALREADY_STOPPED: i64 = -8;
pub spec const SBI_ERR_NO_SHMEM: i64 = -9;

pub spec const SBI_PMU_START_FLAG_SET_INIT_VALUE: u64 = 1;
pub spec const SBI_PMU_START_FLAG_INIT_SNAPSHOT: u64 = 2;

pub spec const SBI_PMU_ERR_INVALID_COUNTER_IDX: i64 = -100;
pub spec const SBI_PMU_ERR_INVALID_FLAGS: i64 = -101;
pub spec const SBI_PMU_ERR_INVALID_VALUE: i64 = -102;
pub spec const SBI_PMU_ERR_INVALID_STATE: i64 = -103;
pub spec const SBI_PMU_ERR_INVALID_SHMEM: i64 = -104;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_MASK: i64 = -105;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_BASE: i64 = -106;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_RANGE: i64 = -107;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_CONFIG: i64 = -108;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_MODE: i64 = -109;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_EVENT: i64 = -110;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_TYPE: i64 = -111;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_WIDTH: i64 = -112;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_SCALE: i64 = -113;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_OFFSET: i64 = -114;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_PERIOD: i64 = -115;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_THRESHOLD: i64 = -116;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_WINDOW: i64 = -117;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_FILTER: i64 = -118;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_GROUP: i64 = -119;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_CLASS: i64 = -120;
pub spec const SBI_PMU_ERR_INVALID_COUNTER_CATEGORY: i64 = -121;

} // verus!
