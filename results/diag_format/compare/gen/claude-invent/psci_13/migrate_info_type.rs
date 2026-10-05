pub open spec fn migrate_info_type_spec(result: i64, old_s: S, new_s: S) -> bool {
    (result == 0 || result == 1 || result == 2 || result == -1)
    && (result == TrustedOsMigrateInfoType(old_s))
    && (TrustedOsMigrateInfoType(new_s) == TrustedOsMigrateInfoType(old_s))
    && (new_s == old_s)
}
