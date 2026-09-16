pub open spec fn rmi_psmmu_activate_spec(psmmu_ptr: Address, params_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Rmm().static.feat_da != FEATURE_TRUE ==> result.status == RMI_ERROR_NOT_SUPPORTED)
  && (!PsmmuAddrIsValid(old_s, psmmu_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (PsmmuAt(old_s, psmmu_ptr).state != PSMMU_INACTIVE ==> result.status == RMI_ERROR_INPUT)
  && (!AddrIsRmiGranuleAligned(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!NonSecureAccessPermitted(old_s, params_ptr) ==> result.status == RMI_ERROR_INPUT)
  && (!Equal(PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg, PsmmuAt(old_s, psmmu_ptr).irq_cfg) ==> result.status == RMI_ERROR_INPUT)
  && (!Equal(PsmmuAt(old_s, psmmu_ptr).flags.cmdq_sync_irq_wired, PsmmuAt(old_s, psmmu_ptr).cmdq_sync_irq_wired) ==> result.status == RMI_ERROR_INPUT)
  && ((PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI && !MsiAddrIsValid(old_s, PsmmuAt(old_s, psmmu_ptr).gerr_addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI && !MsiAddrIsValid(old_s, PsmmuAt(old_s, psmmu_ptr).eventq_addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI && !MsiAddrIsValid(old_s, PsmmuAt(old_s, psmmu_ptr).priq_addr)) ==> result.status == RMI_ERROR_INPUT)
  && ((PsmmuAt(old_s, psmmu_ptr).flags.ats == RMI_FEATURE_TRUE && PsmmuAt(old_s, psmmu_ptr).feat_ats != FEATURE_TRUE) ==> result.status == RMI_ERROR_INPUT)
  && ((PsmmuAt(old_s, psmmu_ptr).flags.pri == RMI_FEATURE_TRUE && PsmmuAt(old_s, psmmu_ptr).feat_pri != FEATURE_TRUE) ==> result.status == RMI_ERROR_INPUT)
  && ((PsmmuAt(old_s, psmmu_ptr).flags.ats == RMI_FEATURE_TRUE && DptL0(old_s).state != DPT_L0_VALID) ==> result.status == RMI_ERROR_DPT)
  && (result.is_Ok() ==> PsmmuAt(new_s, psmmu_ptr).state == PSMMU_ACTIVE)
  && (result.is_Ok() && PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI ==> PsmmuAt(new_s, psmmu_ptr).msi_config.gerr_addr == PsmmuAt(new_s, psmmu_ptr).gerr_addr)
  && (result.is_Ok() && PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI ==> PsmmuAt(new_s, psmmu_ptr).msi_config.gerr_data == PsmmuAt(new_s, psmmu_ptr).gerr_data)
  && (result.is_Ok() && PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI ==> PsmmuAt(new_s, psmmu_ptr).msi_config.eventq_addr == PsmmuAt(new_s, psmmu_ptr).eventq_addr)
  && (result.is_Ok() && PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI ==> PsmmuAt(new_s, psmmu_ptr).msi_config.eventq_data == PsmmuAt(new_s, psmmu_ptr).eventq_data)
  && (result.is_Ok() && PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI ==> PsmmuAt(new_s, psmmu_ptr).msi_config.priq_addr == PsmmuAt(new_s, psmmu_ptr).priq_addr)
  && (result.is_Ok() && PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI ==> PsmmuAt(new_s, psmmu_ptr).msi_config.priq_data == PsmmuAt(new_s, psmmu_ptr).priq_data)
  && ((!(Rmm().static.feat_da != FEATURE_TRUE) &&
       PsmmuAddrIsValid(old_s, psmmu_ptr) &&
       !(PsmmuAt(old_s, psmmu_ptr).state != PSMMU_INACTIVE) &&
       AddrIsRmiGranuleAligned(old_s, params_ptr) &&
       NonSecureAccessPermitted(old_s, params_ptr) &&
       Equal(PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg, PsmmuAt(old_s, psmmu_ptr).irq_cfg) &&
       Equal(PsmmuAt(old_s, psmmu_ptr).flags.cmdq_sync_irq_wired, PsmmuAt(old_s, psmmu_ptr).cmdq_sync_irq_wired) &&
       !((PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI && !MsiAddrIsValid(old_s, PsmmuAt(old_s, psmmu_ptr).gerr_addr))) &&
       !((PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI && !MsiAddrIsValid(old_s, PsmmuAt(old_s, psmmu_ptr).eventq_addr))) &&
       !((PsmmuAt(old_s, psmmu_ptr).flags.irq_cfg == RMI_IRQ_MSI && !MsiAddrIsValid(old_s, PsmmuAt(old_s, psmmu_ptr).priq_addr))) &&
       !((PsmmuAt(old_s, psmmu_ptr).flags.ats == RMI_FEATURE_TRUE && PsmmuAt(old_s, psmmu_ptr).feat_ats != FEATURE_TRUE)) &&
       !((PsmmuAt(old_s, psmmu_ptr).flags.pri == RMI_FEATURE_TRUE && PsmmuAt(old_s, psmmu_ptr).feat_pri != FEATURE_TRUE)) &&
       !((PsmmuAt(old_s, psmmu_ptr).flags.ats == RMI_FEATURE_TRUE && DptL0(old_s).state != DPT_L0_VALID)))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).state == PsmmuAt(old_s, psmmu_ptr).state)
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).msi_config.gerr_addr == PsmmuAt(old_s, psmmu_ptr).msi_config.gerr_addr)
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).msi_config.gerr_data == PsmmuAt(old_s, psmmu_ptr).msi_config.gerr_data)
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).msi_config.eventq_addr == PsmmuAt(old_s, psmmu_ptr).msi_config.eventq_addr)
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).msi_config.eventq_data == PsmmuAt(old_s, psmmu_ptr).msi_config.eventq_data)
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).msi_config.priq_addr == PsmmuAt(old_s, psmmu_ptr).msi_config.priq_addr)
  && (result.is_Err()
    ==> PsmmuAt(new_s, psmmu_ptr).msi_config.priq_data == PsmmuAt(old_s, psmmu_ptr).msi_config.priq_data)
}