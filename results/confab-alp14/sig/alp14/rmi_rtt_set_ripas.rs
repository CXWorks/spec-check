pub open spec fn rmi_rtt_set_ripas_spec(rd: Address, rec_ptr: Address, base: Address, top: Address, result: Result<(), RmiStatusCode>, out_top: Address, old_s: S, new_s: S) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let rec = RecAt(old_s, rec_ptr);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL as int, RMM_RTT_TREE_PRIMARY as int);
    let ripas_pre = walk.rtte.ripas;
    let walk_top_pre = RttSkipEntriesWithRipas(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        walk.level as int,
        base,
        top,
        (rec.ripas_value == RAM) && (rec.ripas_destroyed != CHANGE_DESTROYED),
    );
    (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rec_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rec_ptr).state != REC ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rec.state == REC_RUNNING ==> ResultEqual(result, RMI_ERROR_REC))
    && (rec.owner != rd ==> ResultEqual(result, RMI_ERROR_REC))
    && ((top as int) <= (base as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (base != rec.ripas_addr ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((top as int) > (rec.ripas_top as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(top) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!AddrIsRttLevelAligned(old_s, base, walk.level as int) && ripas_pre != rec.ripas_value)
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (((base as int) == (walk_top_pre as int) && ripas_pre != rec.ripas_value)
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && (AddrRangeIsAuxLive(old_s, base, top, realm_pre)
        ==> ResultEqual(result, RMI_ERROR_RTT(walk.level as int)))
    && ((AddrIsGranuleAligned(rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(rec_ptr)
            && PaIsDelegable(old_s, rec_ptr)
            && GranuleAt(old_s, rec_ptr).state == REC
            && rec.state != REC_RUNNING
            && rec.owner == rd
            && (top as int) > (base as int)
            && base == rec.ripas_addr
            && (top as int) <= (rec.ripas_top as int)
            && AddrIsGranuleAligned(top)
            && (AddrIsRttLevelAligned(old_s, base, walk.level as int) || ripas_pre == rec.ripas_value)
            && ((base as int) != (walk_top_pre as int) || ripas_pre == rec.ripas_value)
            && !AddrRangeIsAuxLive(old_s, base, top, realm_pre))
        ==> (result.is_Ok()
            && RttEntriesInRangeRipas(
                new_s,
                RttAt(new_s, walk.rtt_addr),
                walk.level as int,
                base,
                walk_top_pre,
                rec.ripas_value)
            && RecAt(new_s, rec_ptr).ripas_addr == MinAddress(top, walk_top_pre)
            && out_top == MinAddress(top, walk_top_pre)))
}