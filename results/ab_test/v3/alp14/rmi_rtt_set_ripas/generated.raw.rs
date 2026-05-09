```verus
pub open spec fn rmi_rtt_set_ripas_spec(
    result: RmiCommandReturnCode,
    out_top: Address,
    rd: Address,
    rec_ptr: Address,
    base: Address,
    top: Address,
    old_s: S,
    new_s: S
) -> bool {
    let realm = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let ripas_pre = walk.rtte.ripas;
    let walk_top_pre = RttSkipEntriesWithRipas(
        RttAt(old_s, walk.rtt_addr),
        walk.level,
        base,
        top,
        (rec.ripas_value == RAM) && (rec.ripas_destroyed != CHANGE_DESTROYED)
    );

    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (UInt(top) <= UInt(base) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != rec.ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (UInt(top) > UInt(rec.ripas_top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!AddrIsRttLevelAligned(old_s, base, walk.level) && ripas_pre != rec.ripas_value) 
        ==> result.is_Err() && result.get_Err_0().0 == RMI_ERROR_RTT && result.get_Err_0().1 == walk.level)
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((UInt(base) == UInt(walk_top_pre) && ripas_pre != rec.ripas_value)
        ==> result.is_Err() && result.get_Err_0().0 == RMI_ERROR_RTT && result.get_Err_0().1 == walk.level)
    && (AddrRangeIsAuxLive(old_s, base, top, realm)
        ==> result.is_Err() && result.get_Err_0().0 == RMI_ERROR_RTT && result.get_Err_0().1 == walk.level)
    && ((AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, rec_ptr) && PaIsDelegable(old_s, rec_ptr) && GranuleAt(old_s, rec_ptr).state == REC
        && rec.state != REC_RUNNING && rec.owner == rd
        && UInt(top) > UInt(base) && base == rec.ripas_addr && UInt(top) <= UInt(rec.ripas_top)
        && AddrIsGranuleAligned(old_s, top) && AddrIsRttLevelAligned(old_s, base, walk.level)
        && (UInt(base) != UInt(walk_top_pre) || ripas_pre == rec.ripas_value)
        && !AddrRangeIsAuxLive(old_s, base, top, realm))
        ==> (result == RMI_SUCCESS
            && out_top == MinAddress(top, walk_top_pre)
            && RttEntriesInRangeRipas(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, walk_top_pre, rec.ripas_value)
            && RecAt(new_s, rec_ptr).ripas_addr == MinAddress(top, walk_top_pre)))
}
```