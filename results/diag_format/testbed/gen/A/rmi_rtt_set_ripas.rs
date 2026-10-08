pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let rec_pre = RecAt(old_s, rec_ptr);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL, RMM_RTT_TREE_PRIMARY);
    let ripas_pre = walk.rtte.ripas;
    let walk_top_pre = RttSkipEntriesWithRipas(old_s, RttAt(old_s, walk.rtt_addr), walk.level, base, top, (rec_pre.ripas_value == RAM) && (rec_pre.ripas_destroyed != CHANGE_DESTROYED));
    (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec_pre.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec_pre.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && (top <= base ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != rec_pre.ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (top > rec_pre.ripas_top ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!AddrIsRttLevelAligned(old_s, base, walk.level) && ripas_pre != rec_pre.ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (!AddrIsGranuleAligned(old_s, top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((walk_top_pre == base && ripas_pre != rec_pre.ripas_value) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && (AddrRangeIsAuxLive(old_s, base, top, realm_pre) ==> ResultEqual(result, RMI_ERROR_RTT(walk.level)))
    && ((AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, rec_ptr)
        && PaIsDelegable(old_s, rec_ptr)
        && GranuleAt(old_s, rec_ptr).state == REC
        && rec_pre.state != REC_RUNNING
        && rec_pre.owner == rd
        && top > base
        && base == rec_pre.ripas_addr
        && top <= rec_pre.ripas_top
        && !(!AddrIsRttLevelAligned(old_s, base, walk.level) && ripas_pre != rec_pre.ripas_value)
        && AddrIsGranuleAligned(old_s, top)
        && !(walk_top_pre == base && ripas_pre != rec_pre.ripas_value)
        && !AddrRangeIsAuxLive(old_s, base, top, realm_pre))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(new_s, RttAt(new_s, walk.rtt_addr), walk.level, base, walk_top_pre, rec_pre.ripas_value)
            && RecAt(new_s, rec_ptr).ripas_addr == MinAddress(old_s, top, walk_top_pre)
            && out_top == MinAddress(old_s, top, walk_top_pre)))
}
