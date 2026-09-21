pub open spec fn sbi_pmu_counter_get_info_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error != 0 ==> result.value == 0)
    && (result.error == 0 ==> (result.value as int >= 0))
}