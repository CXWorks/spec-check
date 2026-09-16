pub open spec fn rmi_vdev_destroy_spec(rd: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTracked(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (!PaIsTrackedFine(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (VdevAt(old_s, vdev_ptr).vdev_state != VDEV_UNLOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE))
  && (result.is_Ok() ==> GranuleAt(new_s, vdev_ptr).state == GRAN_DELEGATED)
  && (result.is_Ok() ==> VdevIdIsFree(new_s, RealmAt(new_s, rd), VdevAt(new_s, vdev_ptr).vdev_id))
  && (result.is_Ok() ==> TdiIdIsFree(new_s, VdevAt(new_s, vdev_ptr).tdi_id, PdevAt(new_s, pdev_ptr).routing_id))
  && (result.is_Ok() ==> RealmAt(new_s, rd).num_vdevs == RealmAt(new_s, rd).num_vdevs - 1)
  && (result.is_Ok() ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(new_s, pdev_ptr).num_vdevs - 1)
  && (result.is_Ok() ==> PdevAddrRangeRemoved(new_s, PdevAt(new_s, pdev_ptr), PdevAt(new_s, pdev_ptr), VdevAt(new_s, vdev_ptr).pdev_index))
  && (result.is_Ok() && VdevAt(old_s, vdev_ptr).vsmmu == FEATURE_TRUE ==> VsidIsFree(new_s, VsmmuAt(new_s, VdevAt(new_s, vdev_ptr).vsmmu_addr), VdevAt(new_s, vdev_ptr).vsid))
  && (result.is_Ok() ==> PsmmuStWalk(new_s, PsmmuFromPdev(new_s, PdevAt(new_s, pdev_ptr)), VdevSid(new_s, VdevAt(new_s, vdev_ptr))).ste.state == PSMMU_ST_ENTRY_INVALID)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsTrackedFine(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr) &&
       !(VdevAt(old_s, vdev_ptr).vdev_state != VDEV_UNLOCKED))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> GranuleAt(new_s, vdev_ptr).state == GranuleAt(old_s, vdev_ptr).state)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).realm == VdevAt(old_s, vdev_ptr).realm)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).pdev == VdevAt(old_s, vdev_ptr).pdev)
  && (result.is_Err()
    ==> VdevAt(new_s, vdev_ptr).vdev_state == VdevAt(old_s, vdev_ptr).vdev_state)
  && (result.is_Err()
    ==> RealmAt(new_s, rd).num_vdevs == RealmAt(old_s, rd).num_vdevs)
  && (result.is_Err()
    ==> PdevAt(new_s, pdev_ptr).num_vdevs == PdevAt(old_s, pdev_ptr).num_vdevs)
  && (result.is_Err()
    ==> PsmmuStWalk(new_s, PsmmuFromPdev(new_s, PdevAt(new_s, pdev_ptr)), VdevSid(new_s, VdevAt(new_s, vdev_ptr))).ste.state == PsmmuStWalk(old_s, PsmmuFromPdev(old_s, PdevAt(old_s, pdev_ptr)), VdevSid(old_s, VdevAt(old_s, vdev_ptr))).ste.state)
}