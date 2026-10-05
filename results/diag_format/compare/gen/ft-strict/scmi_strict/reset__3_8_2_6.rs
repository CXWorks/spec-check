pub open spec fn reset__3_8_2_6_spec(domain_id: UInt32, flags: UInt32, reset_state: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!ResetDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (Bits(flags, 31, 3) != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsValidResetFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsSupportedResetState(old_s, domain_id, reset_state) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AgentMayResetDomain(old_s, caller, domain_id) ==> ResultEqual(status, DENIED))
  && (ResetOperationFailed(old_s, domain_id) ==> ResultEqual(status, GENERIC_ERROR))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 1 && Bits(flags, 2, 2) == 0) ==> ResetDomainResetTo(new_s, domain_id, reset_state))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 1 && Bits(flags, 2, 2) == 1) ==> ReturnedOnReceiptOfRequest(new_s, domain_id))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 0 && Bits(flags, 1, 1) == 1) ==> ResetSignalAsserted(new_s, domain_id, reset_state))
  && (ResultEqual(status, SUCCESS) ==> (Bits(flags, 0, 0) == 0 && Bits(flags, 1, 1) == 0) ==> !ResetSignalAsserted(new_s, domain_id, reset_state))
  && ((!(ResetDomainExists(old_s, domain_id)) &&
       !(Bits(flags, 31, 3) != 0) &&
       IsValidResetFlags(old_s, flags) &&
       IsSupportedResetState(old_s, domain_id, reset_state) &&
       AgentMayResetDomain(old_s, caller, domain_id) &&
       !(ResetOperationFailed(old_s, domain_id)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> ResetDomainState(new_s, domain_id) == ResetDomainState(old_s, domain_id))
  && (result != SUCCESS
    ==> ResetSignal(new_s, domain_id) == ResetSignal(old_s, domain_id))
}