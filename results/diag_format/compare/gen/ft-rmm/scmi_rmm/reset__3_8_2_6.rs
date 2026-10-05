pub open spec fn reset__3_8_2_6_spec(domain_id: UInt32, flags: UInt32, reset_state: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!ResetDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsValidResetFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsSupportedResetState(old_s, domain_id, reset_state) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AgentMayResetDomain(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResetOperationFailed(old_s, domain_id) ==> ResultEqual(status, GENERIC_ERROR))
  && (result ==> ResultEqual(status, SUCCESS))
  && (flags[0] == 1 ==> ResetDomainAt(new_s, domain_id) has been reset autonomously by the platform)
  && ((flags[0] == 0 && flags[1] == 1) ==> ResetDomainAt(new_s, domain_id).reset_signal == ASSERTED)
  && ((flags[0] == 0 && flags[1] == 0) ==> ResetDomainAt(new_s, domain_id).reset_signal == DEASSERTED)
  && ((flags[0] == 1 && flags[2] == 1) ==> the platform returns immediately on receipt of the request, and sends a RESET_COMPLETE delayed response (Section 3.8.3.1) when the reset is done)
  && ((!(ResetDomainExists(old_s, domain_id)) &&
       IsValidResetFlags(old_s, flags) &&
       IsSupportedResetState(old_s, domain_id, reset_state) &&
       AgentMayResetDomain(old_s, caller, domain_id) &&
       !(ResetOperationFailed(old_s, domain_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result == SUCCESS && !(flags[0] == 1)
    ==> ResetDomainAt(new_s, domain_id).reset_signal == ResetDomainAt(old_s, domain_id).reset_signal)
  && (result == SUCCESS && (flags[0] == 0 && flags[1] == 0)
    ==> ResetDomainAt(new_s, domain_id).reset_signal == ResetDomainAt(old_s, domain_id).reset_signal)
  && (result == SUCCESS && (flags[0] == 0 && flags[1] == 0)
    ==> ResetDomainAt(new_s, domain_id).reset_signal == ResetDomainAt(old_s, domain_id).reset_signal)
  && (result == SUCCESS && (flags[0] == 1 && flags[2] == 1)
    ==> ResetDomainAt(new_s, domain_id).reset_signal == ResetDomainAt(old_s, domain_id).reset_signal)
}