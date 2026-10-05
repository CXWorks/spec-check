pub open spec fn sbi_pmu_counter_start_spec(result: SbiRet, counter_idx_base: u64, counter_idx_mask: u64, start_flags: u64, initial_value: u64, old_s: S, new_s: S) -> bool {
    (PmuCounterSetHasInvalidCounter(old_s, counter_idx_base, counter_idx_mask)
        ==> result.error == SBI_ERR_INVALID_PARAM)
    && ((!PmuCounterSetHasInvalidCounter(old_s, counter_idx_base, counter_idx_mask)
            && PmuCounterSetHasStartedCounter(old_s, counter_idx_base, counter_idx_mask))
        ==> result.error == SBI_ERR_ALREADY_STARTED)
    && ((!PmuCounterSetHasInvalidCounter(old_s, counter_idx_base, counter_idx_mask)
            && !PmuCounterSetHasStartedCounter(old_s, counter_idx_base, counter_idx_mask)
            && (start_flags & 0x2u64) != 0
            && !PmuSnapshotShmemIsSet(old_s))
        ==> result.error == SBI_ERR_NO_SHMEM)
    && ((result.error != SBI_SUCCESS)
        ==> (forall|idx: u64| PmuCounterInSet(counter_idx_base, counter_idx_mask, idx) ==>
                (PmuCounterIsStarted(new_s, idx) == PmuCounterIsStarted(old_s, idx)
                 && PmuCounterValue(new_s, idx) == PmuCounterValue(old_s, idx))))
    && ((!PmuCounterSetHasInvalidCounter(old_s, counter_idx_base, counter_idx_mask)
            && !PmuCounterSetHasStartedCounter(old_s, counter_idx_base, counter_idx_mask)
            && (start_flags >> 2u64) == 0
            && !((start_flags & 0x1u64) != 0 && (start_flags & 0x2u64) != 0)
            && ((start_flags & 0x2u64) != 0 ==> PmuSnapshotShmemIsSet(old_s)))
        ==> (result.error == SBI_SUCCESS
            && (forall|idx: u64| PmuCounterInSet(counter_idx_base, counter_idx_mask, idx) ==>
                    (PmuCounterIsStarted(new_s, idx)
                     && ((start_flags & 0x1u64) != 0 ==> PmuCounterValue(new_s, idx) == initial_value)
                     && ((start_flags & 0x2u64) != 0 ==> PmuCounterValue(new_s, idx) == PmuSnapshotCounterValue(old_s, idx))
                     && (((start_flags & 0x1u64) == 0 && (start_flags & 0x2u64) == 0)
                         ==> PmuCounterValue(new_s, idx) == PmuCounterValue(old_s, idx)))))))
}
