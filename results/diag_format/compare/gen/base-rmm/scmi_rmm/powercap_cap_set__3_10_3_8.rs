pub open spec fn powercap_cap_set__3_10_3_8_spec(status: int32, old_s: S, new_s: S) -> bool {
    (!IsValidPowercapDomain(old_s, domain_id) ==> ResultEqual(status, NOT_FOUND))
    && (!IsValidCpli(old_s, domain_id, cpli) ==> ResultEqual(status, NOT_FOUND))
    && (!IsRequestSupported(old_s, domain_id, cpli, flags, power_cap) ==> ResultEqual(status, NOT_SUPPORTED))
    && (!IsSupportedPowerCap(old_s, domain_id, cpli, power_cap) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!IsValidCapSetFlags(flags) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (!AgentMaySetPowerCap(agent, domain_id) ==> ResultEqual(status, DENIED))
    && (ResultEqual(status, SUCCESS) ==> (
        (flags[1] == 0 ==> RequestedPowerCap(agent, domain_id, cpli) == power_cap)
        && (flags[1] == 1 ==> CommandQueued(POWERCAP_CAP_SET, domain_id, cpli, power_cap))
        && (flags[1] == 1 && flags[0] == 0 ==> DelayedResponseSent(POWERCAP_CAP_SET_COMPLETE))
        && (flags[1] == 1 && flags[0] == 1 ==> !DelayedResponseSent(POWERCAP_CAP_SET_COMPLETE))
    ))
}