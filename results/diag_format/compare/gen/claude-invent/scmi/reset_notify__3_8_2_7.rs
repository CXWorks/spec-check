pub open spec fn reset_notify__3_8_2_7_spec(status: i32, domain_id: u32, notify_enable: u32, old_s: S, new_s: S) -> bool {
    ((!IsValidResetDomain(old_s, domain_id) && (notify_enable & 0xFFFF_FFFEu32) == 0) ==> status == NOT_FOUND)
    && ((IsValidResetDomain(old_s, domain_id) && (notify_enable & 0xFFFF_FFFEu32) != 0) ==> status == INVALID_PARAMETERS)
    && ((!IsValidResetDomain(old_s, domain_id) && (notify_enable & 0xFFFF_FFFEu32) != 0) ==> (status == NOT_FOUND || status == INVALID_PARAMETERS))
    && (status != SUCCESS ==> new_s == old_s)
    && (status == SUCCESS ==> (
        IsValidResetDomain(old_s, domain_id)
        && (notify_enable & 0xFFFF_FFFEu32) == 0
        && ResetNotifyEnabled(new_s, domain_id) == ((notify_enable & 1u32) == 1u32)
        && (forall|d: u32| d != domain_id ==> ResetNotifyEnabled(new_s, d) == ResetNotifyEnabled(old_s, d))
        && (forall|d: u32| IsValidResetDomain(new_s, d) == IsValidResetDomain(old_s, d))
    ))
}
