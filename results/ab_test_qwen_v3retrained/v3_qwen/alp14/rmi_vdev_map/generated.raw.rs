pub open spec fn rmi_vdev_map_spec(rd: Address, vdev_ptr: Address, ipa: Address, level: Int64, addr: Address, result: Result<RmiCommandReturnCode, RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (!AddrIsGranuleAligned(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegableDevMem(old_s, addr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!GranulesAllState(old_s, addr, ToAddress((addr) + RttLevelSize(old_s, level as int)) as int, DELEGATED) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && ((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 2) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRttLevelAligned(old_s, ipa, level as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level < level ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int)))
  && (RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.state != UNASSIGNED ==> ResultEqual(result, RMI_ERROR_RTT(RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level as int)))
  && (result.is_Ok() ==> GranulesAllState(new_s, addr, ToAddress((addr) + RttLevelSize(new_s, level as int)) as int, DEV_MAPPED))
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.state == ASSIGNED_DEV)
  && (result.is_Ok() ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.addr == addr)
  && (PaIsDelegableNonCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.attr_prot == MEMATTR_NON_CACHEABLE)
  && (PaIsDelegableCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.attr_prot == MEMATTR_PASSTHROUGH)
  && (PaIsDelegableNonCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.sh == SHAREABILITY_OUTER)
  && (PaIsDelegableCohDevMem(old_s, addr) ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.sh == SHAREABILITY_INNER)
  && (result.is_Ok() ==> VdevAt(new_s, vdev_ptr).num_map == VdevAt(new_s, vdev_ptr).num_map + (RttLevelSize(new_s, level as int) << RMM_GRANULE_SIZE_ORDER))
  && ((AddrIsGranuleAligned(old_s, addr) &&
       PaIsDelegableDevMem(old_s, addr) &&
       GranulesAllState(old_s, addr, ToAddress((addr) + RttLevelSize(old_s, level as int)) as int, DELEGATED) &&
       AddrIsGranuleAligned(old_s, rd) &&
       PaIsDelegable(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != RD) &&
       AddrIsGranuleAligned(old_s, vdev_ptr) &&
       PaIsDelegable(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !((!RttLevelIsValid(old_s, RealmAt(old_s, rd), level as int) || level < 2)) &&
       AddrIsRttLevelAligned(old_s, ipa, level as int) &&
       AddrIsProtected(old_s, ipa, RealmAt(old_s, rd)) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).level < level) &&
       !(RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.state != UNASSIGNED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranulesAllState(new_s, addr, ToAddress((addr) + RttLevelSize(new_s, level as int)) as int, GranulesAllState(old_s, addr, ToAddress((addr) + RttLevelSize(old_s, level as int)) as int, DEV_MAPPED)))
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.state == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.state)
  && (result.is_Err()
    ==> RttWalk(new_s, RealmAt(new_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.addr == RttWalk(old_s, RealmAt(old_s, rd), ipa,level as int,RMM_RTT_TREE_PRIMARY).rtte.addr)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).num_map == VdevAt(old_s, vdev_ptr).num_map)
}