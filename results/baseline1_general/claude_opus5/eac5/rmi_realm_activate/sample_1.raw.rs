pub open spec fn rmi_realm_activate_spec(result: Result<(), RmiStatusCode>, rd: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (Granule(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && Realm(old_s, rd).state != REALM_NEW) ==> ResultEqual(result, RMI_ERROR_REALM))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && Granule(old_s, rd).state == RD
        && Realm(old_s, rd).state == REALM_NEW) ==> (result.is_Ok() && Realm(new_s, rd).state == REALM_ACTIVE))
}