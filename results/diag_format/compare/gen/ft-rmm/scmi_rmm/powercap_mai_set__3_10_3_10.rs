pub open spec fn powercap_mai_set__3_10_3_10_spec(domain_id: UInt32, flags: UInt32, mai: UInt32, status: Int32, old_s: S, new_s: S) -> bool {
  (!PowercapDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
  && (!IsMaiSetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!AreValidMaiSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!IsSupportedMai(old_s, domain_id, mai) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (!AgentMaySetMai(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> PowercapDomain(new_s, domain_id).mai == mai)
  && ((!(PowercapDomainExists(old_s, domain_id)) &&
       IsMaiSetSupported(old_s, domain_id) &&
       AreValidMaiSetFlags(old_s, flags) &&
       IsSupportedMai(old_s, domain_id, mai) &&
       AgentMaySetMai(old_s, calling_agent, domain_id))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> PowercapDomain(new_s, domain_id).mai == PowercapDomain(old_s, domain_id).mai)
}