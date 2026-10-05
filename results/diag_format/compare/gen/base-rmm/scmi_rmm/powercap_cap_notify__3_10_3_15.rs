pub open spec fn powercap_cap_notify__3_10_3_15_spec(result: int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(result, NOT_FOUND))
    && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (ResultEqual(result, SUCCESS) ==> PowercapCapNotifyEnabled(old_s, calling_agent) == notify_enable[0])
}