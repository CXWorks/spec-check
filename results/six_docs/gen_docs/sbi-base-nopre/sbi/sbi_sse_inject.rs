pub open spec fn sbi_sse_inject_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result == SBI_SUCCESS ==> <success postconditions>)
    && (result != SBI_SUCCESS ==> <failure postconditions>)
}