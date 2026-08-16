pub open spec fn rmi_realm_destroy_spec(
    result: Result<(), RmiStatusCode>,
    rd: Address,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state
        == RD && RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM)) && (
    AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && Granule(old_s, rd).state == RD
        && !RealmIsLive(old_s, rd) ==> result.is_Ok() && RttsStateEqual(
        Realm(old_s, rd).rtt_base,
        Realm(old_s, rd).rtt_num_start,
        DELEGATED,
    ) && Granule(new_s, rd).state == DELEGATED && VmidIsFree(new_s, Realm(old_s, rd).vmid))
}