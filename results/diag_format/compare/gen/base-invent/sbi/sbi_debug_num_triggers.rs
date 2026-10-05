pub open spec fn sbi_debug_num_triggers_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error() ==> true)
    && (result.ok() ==> true)
}