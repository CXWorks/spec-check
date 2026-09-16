pub open spec fn rmi_vsmmu_cmd_complete_spec(rd: Address, vsmmu_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, flags: RmiVsmmuCmdFlags, vsid: UInt64, msi_addr: Address, msi_data: UInt64, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_vsmmu != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result.status == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result.status == RMI_ERROR_REALM(0))
  && (!AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vsmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, pdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vdev_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).vsmmu_addr != vsmmu_ptr ==> result.status == RMI_ERROR_INPUT)
  && (VsmmuAt(old_s, vsmmu_ptr).realm != rd ==> result.status == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> result.status == RMI_ERROR_DEVICE)
  && ((!(Rmm().static.feat_vsmmu != FEATURE_TRUE) &&
       AddrIsRmiGranuleAligned(old_s, rd) &&
       PaIsTracked(old_s, rd) &&
       !(GranuleAt(old_s, rd).state != GRAN_RD) &&
       !(RealmAt(old_s, rd).state != REALM_NEW) &&
       AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) &&
       PaIsTracked(old_s, vsmmu_ptr) &&
       !(GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU) &&
       AddrIsRmiGranuleAligned(old_s, pdev_ptr) &&
       PaIsTracked(old_s, pdev_ptr) &&
       !(GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV) &&
       AddrIsRmiGranuleAligned(old_s, vdev_ptr) &&
       PaIsTracked(old_s, vdev_ptr) &&
       !(GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV) &&
       !(VdevAt(old_s, vdev_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_ptr).vsmmu_addr != vsmmu_ptr) &&
       !(VsmmuAt(old_s, vsmmu_ptr).realm != rd) &&
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr))
    ==> result.is_Ok())
}