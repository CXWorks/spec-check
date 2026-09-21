pub open spec fn migrate_info_type_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == 0 ==> (CpuIsValid(old_s, 0) && CpuIsOn(old_s, 0) && !TrustedOsMigrationInProgress(old_s)))
    && (result == 1 ==> (CpuIsValid(old_s, 0) && CpuIsOn(old_s, 0) && !TrustedOsMigrationInProgress(old_s)))
    && (result == 2 ==> (CpuIsValid(old_s, 0) && !TrustedOsMigrationInProgress(old_s)))
    && (result != 0 && result != 1 && result != 2 ==> true)
    && (result == 0 ==> new_s.cpu_state == old_s.cpu_state)
    && (result == 1 ==> new_s.cpu_state == old_s.cpu_state)
    && (result == 2 ==> new_s.cpu_state == old_s.cpu_state)
}