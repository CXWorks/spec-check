```verus
pub open spec fn rmi_rtt_init_ripas_spec(
    result: RmiCommandReturnCode,
    out_top: Address,
    old_s: S,
    new_s: S,
    rd: Address,
    base: Address,
    top: Address
) -> bool {
    let realm_pre = RealmAt(old_s, rd);
    let walk = RttWalk(old_s, realm_pre, base, RMM_RTT_PAGE_LEVEL);
    let walk_top = RttSkipEntriesIfNotState(
        old_s,
        RttAt(old_s, walk.rtt_addr),
        walk.level,
        base,
        top,
        UNASSIGNED
    );
    let realm = RealmAt(new_s, rd);
    
    // Failure conditions with ordering constraints
    (
        // rd_align: rd must be granule aligned
        (!AddrIsGranuleAligned(rd) ==> result == RMI_ERROR_INPUT)
    ) && (
        // rd_bound: rd must be delegable
        (!PaIsDelegable(rd) ==> result == RMI_ERROR_INPUT)
    ) && (
        // size_valid: top must be greater than base
        (UInt(top) <= UInt(base) ==> result == RMI_ERROR_INPUT)
    ) && (
        // top_gran_align: top must be granule aligned
        (!AddrIsGranuleAligned(top) ==> result == RMI_ERROR_INPUT)
    ) && (
        // rd_state: RD granule must be in RD state (checked before realm_state)
        (GranuleAt(old_s, rd).state != RD ==> result == RMI_ERROR_INPUT)
    ) && (
        // top_bound: (top - RMM_GRANULE_SIZE) must be protected
        (!AddrIsProtected(
            old_s,
            ToAddress(UInt(top) - RMM_GRANULE_SIZE),
            realm_pre
        ) ==> result == RMI_ERROR_INPUT)
    ) && (
        // realm_state: realm must be in REALM_NEW state (checked after rd validation)
        (realm_pre.state != REALM_NEW ==> result == RMI_ERROR_REALM)
    ) && (
        // base_align: base must be aligned to RTT level
        (!AddrIsRttLevelAligned(old_s, base, walk.level) ==> 
            result == RmiErrorRtt(walk.level))
    ) && (
        // rtte_state: RTT entry at walk location must be UNASSIGNED
        (walk.rtte.state != UNASSIGNED ==> 
            result == RmiErrorRtt(walk.level))
    ) && (
        // no_progress: walk_top must be greater than base
        (UInt(base) == UInt(walk_top) ==> 
            result == RmiErrorRtt(walk.level))
    ) && (
        // Success conditions
        (
            AddrIsGranuleAligned(rd) &&
            PaIsDelegable(rd) &&
            GranuleAt(old_s, rd).state == RD &&
            UInt(top) > UInt(base) &&
            AddrIsGranuleAligned(top) &&
            AddrIsProtected(
                old_s,
                ToAddress(UInt(top) - RMM_GRANULE_SIZE),
                realm_pre
            ) &&
            realm_pre.state == REALM_NEW &&
            AddrIsRttLevelAligned(old_s, base, walk.level) &&
            walk.rtte.state == UNASSIGNED &&
            UInt(base) < UInt(walk_top)
        ) ==> (
            result == RMI_SUCCESS &&
            // rtte_ripas: all RTT entries in range have RIPAS set to RAM
            RttEntriesInRangeRipas(
                old_s,
                RttAt(old_s, walk.rtt_addr),
                walk.level,
                base,
                walk_top,
                RAM
            ) &&
            // rim: RIM is extended with RIPAS data
            realm.measurements[0] == RimExtendRipas(
                old_s,
                realm_pre,
                base,
                walk_top,
                walk.level
            ) &&
            // out_top: output top matches walk_top
            out_top == walk_top
        )
    )
}
```