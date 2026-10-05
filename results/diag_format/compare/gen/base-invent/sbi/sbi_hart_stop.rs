pub open spec fn sbi_hart_stop_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.is_err() ==> true)
    && (result.is_ok() ==> true)
}