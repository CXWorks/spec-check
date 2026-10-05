pub open spec fn sbi_pmu_counter_get_info_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error != 0 ==> true)
    && (result.error == 0 ==> (result.value as int >= 0 && result.value as int < (1u64 << 12) && (result.value as int) < (1u64 << 18) && (result.value as int) < (1u64 << (XLEN - 2)) && (result.value as int) < (1u64 << XLEN)))
}