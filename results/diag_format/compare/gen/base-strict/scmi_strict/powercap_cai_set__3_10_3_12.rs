pub open spec fn powercap_cai_set__3_10_3_12_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomainId(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(result, NOT_FOUND))
    && (!IsCaiSetSupported(old_s, domain_id) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsSupportedCai(old_s, domain_id, cai) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!AgentMaySetCai(old_s, CallingAgent(), domain_id) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> PowercapCai(new_s, domain_id, cpli) == cai)
}