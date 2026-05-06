pub open spec fn rmi_realm_activate_spec(
    result: RmiCommandReturnCode,
    rd: Address,
    old_s: S,
    new_s: S,
) -> bool {
    // Failure condition: rd_align
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Failure condition: rd_bound
     && (!PaIsDelegable(rd) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Failure condition: rd_state
     && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Failure condition: realm_state
     && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(
        result,
        RMI_ERROR_REALM,
    ))
    // Success condition: realm_state
     && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD
        && RealmAt(old_s, rd).state == REALM_NEW) ==> (result.is_Ok() && RealmAt(new_s, rd).state
        == REALM_ACTIVE))
}