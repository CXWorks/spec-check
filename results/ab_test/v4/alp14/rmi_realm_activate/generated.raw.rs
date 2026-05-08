pub open spec fn RMI_REALM_ACTIVATE_spec(old_s: S, new_s: S, rd: Address, result: RmiCommandReturnCode) -> bool {
    let realm = RealmAt(old_s, rd);
    (
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (realm.state != REALM_NEW ==> ResultEqual(result, RMI_ERROR_REALM)) &&
        (
            AddrIsGranuleAligned(rd) &&
            PaIsDelegable(rd) &&
            GranuleAt(old_s, rd).state == RD &&
            realm.state == REALM_NEW
        ) ==> (
            RealmAt(new_s, rd).state == REALM_ACTIVE &&
            result.is_Ok()
        )
    )
}