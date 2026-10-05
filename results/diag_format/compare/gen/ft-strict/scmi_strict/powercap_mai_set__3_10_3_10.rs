pub open spec fn powercap_mai_set__3_10_3_10_spec(domain_id: UInt32, flags: UInt32, mai: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!PowercapDomainExists(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
  && (!IsMaiSetSupported(old_s, domain_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!IsSupportedMai(old_s, domain_id, mai) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AreValidMaiSetFlags(old_s, flags) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!AgentMaySetMai(old_s, calling_agent, domain_id) ==> ResultEqual(status, DENIED))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> PowercapDomainMai(new_s, domain_id) == mai)
  && ((PowercapDomainExists(old_s, domain_id) &&
       IsMaiSetSupported(old_s, domain_id) &&
       IsSupportedMai(old_s, domain_id, mai) &&
       AreValidMaiSetFlags(old_s, flags) &&
       AgentMaySetMai(old_s, calling_agent, domain_id))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PowercapDomainMai(new_s, domain_id) == PowercapDomainMai(old_s, domain_id))
}