pub open spec fn powercap_mai_set__3_10_3_10_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!PowercapDomainExists(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsMaiSetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AreValidMaiSetFlags(old_s, flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSupportedMai(old_s, domain_id, mai) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AgentMaySetMai(old_s, calling_agent, domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> PowercapDomain(new_s, domain_id).mai == mai)
}