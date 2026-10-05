pub open spec fn reset_complete__3_8_3_1_spec(domain_id: UInt32, status: Int32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (ResetDomainHasOtherUsers(old_s, domain_id) ==> ResultEqual(result, GENERIC_ERROR))
  && (!ResetDomainCanBeQuiesced(old_s, domain_id) ==> ResultEqual(result, GENERIC_ERROR))
  && (ResetOperationFailed(old_s, domain_id) ==> ResultEqual(result, GENERIC_ERROR))
  && (result.is_Ok() ==> ResultEqual(status, SUCCESS))
  && (result.is_Ok() ==> domain_id == AsyncResetRequestedDomain(new_s))
  && (result.is_Ok() ==> ResetDomainWasReset(new_s, domain_id))
  && ((!ResetDomainHasOtherUsers(old_s, domain_id) &&
       ResetDomainCanBeQuiesced(old_s, domain_id) &&
       !ResetOperationFailed(old_s, domain_id))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> ResetDomainState(new_s, domain_id) == ResetDomainState(old_s, domain_id))
}