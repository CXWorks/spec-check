pub open spec fn sbi_remote_fence_i_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.error == SBI_ERR_INVALID_PARAM ==> <invalid param failure condition>)
    && (result.error == SBI_ERR_FAILED ==> <failed failure condition>)
    && (result.error == SBI_SUCCESS ==> <success postconditions>)
}