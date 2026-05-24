pub open spec fn rmi_realm_activate_spec(result: RmiCommandReturnCode, rd: Address, old_s: S, new_s: S) -> bool {
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (RealmAt(old_s, rd).state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && ((AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD && RealmAt(old_s, rd).state == REALM_NEW) ==> (result.is_Ok() && RealmAt(new_s, rd).state == REALM_ACTIVE))
}