```verus
pub open spec fn rmi_vdev_map_spec(
    result: RmiCommandReturnCode,
    old_s: S,
    new_s: S,
    rd: Address,
    vdev_ptr: Address,
    ipa: Address,
    level: int,
    addr: Address
) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, RMM_RTT_TREE_PRIMARY);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let pa_top = ToAddress(UInt(addr) + RttLevelSize(old_s, level));

    // Failure conditions
    let addr_align_fail = !AddrIsGranuleAligned(addr);
    let addr_bound_fail = !PaIsDelegableDevMem(addr);
    let addr_state_fail = !GranulesAllState(old_s, addr, pa_top, DELEGATED);
    let rd_align_fail = !AddrIsGranuleAligned(rd);
    let rd_bound_fail = !PaIsDelegable(rd);
    let rd_state_fail = GranuleAt(old_s, rd).state != RD;
    let vdev_align_fail = !AddrIsGranuleAligned(vdev_ptr);
    let vdev_bound_fail = !PaIsDelegable(vdev_ptr);
    let vdev_gran_state_fail = GranuleAt(old_s, vdev_ptr).state != VDEV;
    let vdev_realm_fail = vdev_pre.realm != rd;
    let level_bound_fail = (!RttLevelIsValid(old_s, realm, level) || level < 2);
    let ipa_align_fail = !AddrIsRttLevelAligned(ipa, level);
    let ipa_bound_fail = !AddrIsProtected(ipa, realm);
    let rtt_walk_fail = walk.level < level;
    let rtte_state_fail = walk.rtte.state != UNASSIGNED;

    // Failure condition dependencies (ordering constraints)
    let rd_checked = !rd_align_fail && !rd_bound_fail;
    let vdev_checked = !vdev_align_fail && !vdev_bound_fail && !vdev_gran_state_fail;
    let level_ipa_checked = !level_bound_fail && !ipa_align_fail && !ipa_bound_fail;

    // Check failures in order
    (addr_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (addr_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (addr_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rd_state_fail && rd_checked ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_gran_state_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_realm_fail && vdev_checked ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (level_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_align_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (ipa_bound_fail ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (rtt_walk_fail && rd_checked && level_ipa_checked ==> ResultEqual(result, RMI_ERROR_RTT))
    && (rtte_state_fail && rd_checked && level_ipa_checked ==> ResultEqual(result, RMI_ERROR_RTT))

    // Success conditions
    && (!addr_align_fail && !addr_bound_fail && !addr_state_fail
        && !rd_align_fail && !rd_bound_fail && !rd_state_fail
        && !vdev_align_fail && !vdev_bound_fail && !vdev_gran_state_fail && !vdev_realm_fail
        && !level_bound_fail && !ipa_align_fail && !ipa_bound_fail
        && !rtt_walk_fail && !rtte_state_fail
        ==> (result.is_Ok()
            && GranulesAllState(new_s, addr, pa_top, DEV_MAPPED)
            && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED_DEV
            && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == addr
            && (PaIsDelegableNonCohDevMem(addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_prot == MEMATTR_NON_CACHEABLE)
            && (PaIsDelegableCohDevMem(addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_prot == MEMATTR_PASSTHROUGH)
            && (PaIsDelegableNonCohDevMem(addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).sh == SHAREABILITY_OUTER)
            && (PaIsDelegableCohDevMem(addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).sh == SHAREABILITY_INNER)
            && VdevAt(new_s, vdev_ptr).num_map == vdev_pre.num_map + (RttLevelSize(old_s, level) << RMM_GRANULE_SIZE_ORDER)))
}
```