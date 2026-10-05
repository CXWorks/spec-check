pub open spec fn reset_issued__3_8_4_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!IsRegisteredForResetNotification(old_s, agent_id, domain_id) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (IsRegisteredForResetNotification(old_s, agent_id, domain_id) ==> result.is_Ok() && ResetDomainHasBeenReset(old_s, domain_id, reset_state))
}