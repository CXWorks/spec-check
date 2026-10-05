pub open spec fn migrate_info_up_cpu_spec(result: UInt64, old_s: S, new_s: S) -> bool {
    (((MigrateInfoType(old_s) == 0) || (MigrateInfoType(old_s) == 1)) ==> (result == TrustedOsResidentMpidr(old_s)))
    && (new_s == old_s)
}
