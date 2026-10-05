pub open spec fn reset_complete__3_8_3_1_spec(status: i32, domain_id: u32, old_s: S, new_s: S) -> bool {
    (!IsResetSuccessful(old_s, domain_id) ==> status != SUCCESS)
    && (IsResetSuccessful(old_s, domain_id) ==> status == SUCCESS)
}
