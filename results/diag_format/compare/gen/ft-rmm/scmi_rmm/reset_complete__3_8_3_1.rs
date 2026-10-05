pub open spec fn reset_complete__3_8_3_1_spec(domain_id: UInt32, status: Int32, domain_reset: ResetDomainAt, old_s: S, new_s: S) -> bool {
  (ResetDomainHasOtherUsers(old_s, domain_id) ==> ResultEqual(status, GENERIC_ERROR))
  && (!ResetDomainCanBeQuiesced(old_s, domain_id) ==> ResultEqual(status, GENERIC_ERROR))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> ResetDomainWasReset(new_s, domain_id))
  && ((!ResetDomainHasOtherUsers(old_s, domain_id) &&
       ResetDomainCanBeQuiesced(old_s, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result: Result<(), RmiStatusCode>,
       !(ResetDomainHasOtherUsers(old_s, domain_id)) &&
       ResetDomainCanBeQuiesced(old_s, domain_id))
    ==> ResultEqual(result, RMI_SUCCESS))
}