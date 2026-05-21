pub open spec fn rmi_realm_activate_spec(result: RmiCommandReturnCode, rd: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (realm.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM))
    && (AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD && realm.state == REALM_NEW ==> (result == RMI_OK && RealmAt(new_s, rd).state == REALM_ACTIVE))
}