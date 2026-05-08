pub open spec fn RMI_REALM_ACTIVATE_spec(
    s: S,
    rd: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let realm = RealmAt(s, rd);
    (!AddrIsGranuleAligned(s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (!PaIsDelegable(
        s,
        rd,
    ) ==> ResultEqual(result, RMI_ERROR_INPUT)) && (GranuleAt(s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    )) && (realm.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM)) && ((
    AddrIsGranuleAligned(s, rd) && PaIsDelegable(s, rd) && GranuleAt(s, rd).state == RD
        && realm.state == REALM_NEW) ==> result.is_Ok())
}