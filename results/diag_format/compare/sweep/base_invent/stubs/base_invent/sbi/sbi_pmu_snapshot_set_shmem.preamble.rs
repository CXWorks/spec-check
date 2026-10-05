use vstd::prelude::*;
verus! {

pub struct sbiret {
    pub code: i64,
    pub value: u64,
}

pub struct S {
    pub cmd_input_shmem_phys_lo: u64,
    pub cmd_input_shmem_phys_hi: u64,
    pub pmu_snapshot_shmem_addr: u64,
    pub pmu_snapshot_shmem_enabled: bool,
    pub pmu_snapshot_shmem_size: u64,
}

} // verus!
