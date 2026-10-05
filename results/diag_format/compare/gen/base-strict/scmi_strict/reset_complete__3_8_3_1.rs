pub open spec fn reset_complete__3_8_3_1_spec(result: Result<(), RmiStatusCode>, status: Int32, domain_id: UInt32, old_s: S, new_s: S) -> bool {
    (ResetDomainHasOtherUsers(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_GENERIC))
    && (!ResetDomainCanBeQuiesced(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_GENERIC))
    && (ResetOperationFailed(old_s, domain_id) ==> ResultEqual(result, RMI_ERROR_GENERIC))
    && (result.is_Ok() ==> ResultEqual(status, SUCCESS))
    && (result.is_Ok() ==> domain_id == AsyncResetRequestedDomain())
    && (result.is_Ok() ==> ResetDomainWasReset(old_s, domain_id))
    && (result.is_Ok() ==> ResetDomainState(old_s, domain_id) == ResetDomainState(new_s, domain_id))
}