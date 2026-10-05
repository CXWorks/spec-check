pub open spec fn reset_complete__3_8_3_1_spec(result: Result<(), RmiStatusCode>, status: i32, domain_id: u32, old_s: S, new_s: S) -> bool {
    (ResetDomainHasOtherUsers(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_GENERIC))
    && (!ResetDomainCanBeQuiesced(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_GENERIC))
    && (result.is_Ok() ==> ResultEqual(status, 0))
    && (result.is_Ok() ==> ResetDomainWasReset(new_s, domain_id))
}