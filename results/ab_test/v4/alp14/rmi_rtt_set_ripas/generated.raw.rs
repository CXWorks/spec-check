```rust
pub open spec fn RMI_RTT_SET_RIPAS_spec(
    old_s: S,
    new_s: S,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
    result: RmiCommandReturnCode,
    out_top: Address,
) -> bool {
    let realm = RealmAt(old_s, rd);
    let realm_pre = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let ripas_pre = walk.rtte.ripas;
    let walk_top_pre = RttSkipEntriesWithRipas(
        RttAt(old_s, walk.rtt_addr),
        walk.level,
        base,
        top,
        (rec.ripas_value == RAM) && (rec.ripas_destroyed != CHANGE_DESTROYED),
    );

    // Failure conditions
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (!PaIsDelegable(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC)) &&
    (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC)) &&
    (UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (base != rec.ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    (UInt(top) > UInt(rec.ripas_top) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((!AddrIsRttLevelAligned(base, walk.level) && ripas_pre != rec.ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT)) &&
    (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    ((UInt(base) == UInt(walk_top_pre) && ripas_pre != rec.ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT)) &&
    (AddrRangeIsAuxLive(old_s, base, top, realm_pre) ==> ResultEqual(result, RMI_ERROR_RTT)) &&

    // Success conditions
    (result == RMI_SUCCESS ==>
        RttEntriesInRangeRipas(RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top_pre, rec.ripas_value) &&
        RecAt(new_s, rec_ptr).ripas_addr == MinAddress(top, walk_top_pre) &&
        out_top == MinAddress(top, walk_top_pre)
    )
}
```