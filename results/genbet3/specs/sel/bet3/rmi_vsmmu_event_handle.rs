pub open spec fn rmi_vsmmu_event_handle_spec(rd: Address, vsmmu_ptr: Address, pdev_ptr: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, flags: RmiVsmmuEventFlags, msi_addr: Address, msi_data: Bits64, fipa: Address, syndrome: Bits64, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_vsmmu != FEATURE_TRUE ==> result == RMI_ERROR_NOT_SUPPORTED)
  && (!AddrIsRmiGranuleAligned(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, rd) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, rd).state != GRAN_RD ==> result == RMI_ERROR_INPUT)
  && (RealmAt(old_s, rd).state != REALM_NEW ==> result == RMI_ERROR_REALM(0))
  && (!AddrIsRmiGranuleAligned(old_s, vsmmu_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vsmmu_ptr) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vsmmu_ptr).state != GRAN_VSMMU ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, pdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, pdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, pdev_ptr).state != GRAN_PDEV ==> result == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, vdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (!PaIsTracked(old_s, vdev_ptr) ==> result == RMI_ERROR_INPUT)
  && (GranuleAt(old_s, vdev_ptr).state != GRAN_VDEV ==> result == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).realm != rd ==> result == RMI_ERROR_INPUT)
  && (VdevAt(old_s, vdev_ptr).pdev != pdev_ptr ==> result == RMI_ERROR_DEVICE)
  && (result.is_Ok() && VsmmuAt(old_s, vsmmu_ptr).gerr_irq ==> (flags.irq == RMI_TRUE && msi_addr == VsmmuAt(old_s, vsmmu_ptr).msi_config.gerr_addr && msi_data == VsmmuAt(old_s, vsmmu_ptr).msi_config.gerr_data))
  && (result.is_Ok() && VsmmuAt(old_s, vsmmu_ptr).priq_irq ==> (flags.irq == RMI_TRUE && msi_addr == VsmmuAt(old_s, vsmmu_ptr).msi_config.priq_addr && msi_data == VsmmuAt(old_s, vsmmu_ptr).msi_config.priq_data))
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
       !(VdevAt(old_s, vdev_ptr).pdev != pdev_ptr))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).gerr_irq == VsmmuAt(old_s, vsmmu_ptr).gerr_irq)
  && (result.is_Err()
    ==> VsmmuAt(new_s, vsmmu_ptr).priq_irq == VsmmuAt(old_s, vsmmu_ptr).priq_irq)
}