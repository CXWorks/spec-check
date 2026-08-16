pub open spec fn rmi_realm_destroy_spec(
    rd: Address,
    result: Result<(), RmiStatusCode>,
    old_s: S,
    new_s: S,
) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        old_s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (Granule(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (RealmIsLive(old_s, rd) ==> ResultEqual(result, RMI_ERROR_REALM(0))) && (result.is_Ok()
        ==> RttsStateEqual(
        Realm(new_s, rd).rtt_base,
        Realm(new_s, rd).rtt_num_start as int,
        DELEGATED,
    )) && (result.is_Ok() ==> Granule(new_s, rd).state == DELEGATED) && (result.is_Ok()
        ==> VmidIsFree(new_s, Realm(new_s, rd).vmid)) && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd) && !(Granule(old_s, rd).state != RD) && !(RealmIsLive(
        old_s,
        rd,
    ))) ==> result.is_Ok()) && (result.is_Err() ==> Granule(new_s, rd).state == Granule(
        old_s,
        rd,
    ).state)
}