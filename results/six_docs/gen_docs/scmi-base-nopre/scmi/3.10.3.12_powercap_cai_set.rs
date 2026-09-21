pub open spec fn 3.10.3.12_powercap_cai_set_spec(result: int32, old_s: S, new_s: S) -> bool {
    (flags(old_s) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (cai(old_s) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && (domain_id(old_s) is_invalid ==> ResultEqual(result, NOT_FOUND))
    && (cpli(old_s) is_invalid ==> ResultEqual(result, NOT_FOUND))
    && (request_is_not_supported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (agent_not_allowed(old_s) ==> ResultEqual(result, DENIED))
    && (cai_not_supported(old_s) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (result == SUCCESS ==> cai(new_s) == cai(old_s))
    && (result == SUCCESS ==> domain_id(new_s) == domain_id(old_s))
    && (result == SUCCESS ==> flags(new_s) == flags(old_s))
    && (result == SUCCESS ==> cpli(new_s) == cpli(old_s))
}