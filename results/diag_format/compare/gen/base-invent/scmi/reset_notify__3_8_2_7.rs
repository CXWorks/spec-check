pub open spec fn reset_notify__3_8_2_7_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_FOUND ==> !IsDomainValid(old_s, domain_id(old_s)))
    && (result == INVALID_PARAMETERS ==> (notify_enable(old_s) & 0xFFFFFFFE != 0))
    && (result == SUCCESS ==> (notify_enable(old_s) & 0x1 == notify_enable(new_s) & 0x1))
    && (result == SUCCESS ==> (domain_id(old_s) == domain_id(new_s)))
}