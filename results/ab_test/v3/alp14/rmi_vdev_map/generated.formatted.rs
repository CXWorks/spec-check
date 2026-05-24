pub open spec fn rmi_vdev_map_spec(result: RmiCommandReturnCode, rd: Address, vdev_ptr: Address, ipa: Address, level: int, addr: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev_pre = VdevAt(old_s, vdev_ptr);
    let walk = RttWalk(old_s, realm, ipa, level, 0);
    let entry_idx = RttEntryIndex(old_s, ipa, walk.level);
    let pa_top = ToAddress((addr as int) + RttLevelSize(old_s, level));

    (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegableDevMem(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranulesAllState(old_s, addr, pa_top, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev_pre.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((!RttLevelIsValid(old_s, realm, level) || level < 2) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsRttLevelAligned(old_s, ipa, level) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsProtected(old_s, ipa, realm) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (walk.level < level ==> ResultEqual(result, RMI_ERROR_RTT))
    && (walk.rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT))
    && ((AddrIsGranuleAligned(old_s, addr)
        && PaIsDelegableDevMem(old_s, addr)
        && GranulesAllState(old_s, addr, pa_top, DELEGATED)
        && AddrIsGranuleAligned(old_s, rd)
        && PaIsDelegable(old_s, rd)
        && GranuleAt(old_s, rd).state == RD
        && AddrIsGranuleAligned(old_s, vdev_ptr)
        && PaIsDelegable(old_s, vdev_ptr)
        && GranuleAt(old_s, vdev_ptr).state == VDEV
        && vdev_pre.realm == rd
        && RttLevelIsValid(old_s, realm, level)
        && level >= 2
        && AddrIsRttLevelAligned(old_s, ipa, level)
        && AddrIsProtected(old_s, ipa, realm)
        && walk.level >= level
        && walk.rtte.state == UNASSIGNED)
        ==> (result.is_Ok()
            && GranulesAllState(new_s, addr, pa_top, DEV_MAPPED)
            && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).state == ASSIGNED_DEV
            && RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).addr == addr
            && (PaIsDelegableNonCohDevMem(old_s, addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_prot == MEMATTR_NON_CACHEABLE)
            && (PaIsDelegableCohDevMem(old_s, addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).attr_prot == MEMATTR_PASSTHROUGH)
            && (PaIsDelegableNonCohDevMem(old_s, addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).sh == SHAREABILITY_OUTER)
            && (PaIsDelegableCohDevMem(old_s, addr) ==> RttEntryAt(new_s, RttAt(new_s, walk.rtt_addr), entry_idx).sh == SHAREABILITY_INNER)
            && VdevAt(new_s, vdev_ptr).num_map == vdev_pre.num_map + (RttLevelSize(old_s, level) << 12)))
}