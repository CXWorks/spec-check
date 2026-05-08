pub open spec fn RMI_REALM_ACTIVATE_spec(old_s: S, new_s: S, rd: Address, result: Result<(), RmiStatusCode>) -> bool {
    let realm = RealmAt(old_s, rd);
    (
        // rd_align failure
        (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        // rd_bound failure
        (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        // rd_state failure
        (GranuleAt(old_s, rd).state != RD() ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
        // realm_state failure
        (realm.state != REALM_NEW() ==> ResultEqual(result, RMI_ERROR_REALM())) &&
        // success condition
        (
            (AddrIsGranuleAligned(rd) && PaIsDelegable(rd) && GranuleAt(old_s, rd).state == RD() && realm.state == REALM_NEW())
            ==> (result.is_Ok() && RealmAt(new_s, rd).state == REALM_ACTIVE())
        )
    )
}