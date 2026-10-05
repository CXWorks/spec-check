pub open spec fn powercap_cap_notify__3_10_3_15_spec(status: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidNotifyEnable(old_s, notify_enable) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (ResultEqual(status, SUCCESS) ==> (
        ((Bits(notify_enable, 0, 0) == 1) ==> CapChangeNotifyEnabled(caller, domain_id))
        && ((Bits(notify_enable, 0, 0) == 0) ==> !CapChangeNotifyEnabled(caller, domain_id))
    ))
}