pub open spec fn sbi_pmu_counter_get_info_spec(counter_idx: UInt64, ret: SbiRet, old_s: S, new_s: S) -> bool {
    (!PmuCounterIsValid(old_s, counter_idx as int) ==> (ret.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && (PmuCounterIsValid(old_s, counter_idx as int) ==> (
        ret.error == SBI_SUCCESS
        && new_s == old_s
        && (PmuCounterIsFirmware(old_s, counter_idx as int) ==> ((ret.value >> 63u64) as int) == 1)
        && (!PmuCounterIsFirmware(old_s, counter_idx as int) ==> (
            ((ret.value >> 63u64) as int) == 0
            && ((ret.value & 0xFFFu64) as int) == PmuCounterCsr(old_s, counter_idx as int) as int
            && (((ret.value >> 12u64) & 0x3Fu64) as int) == (PmuCounterNumBits(old_s, counter_idx as int) as int) - 1
        ))
    ))
}
