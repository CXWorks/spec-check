pub open spec fn rmi_pdev_ide_key_refresh_spec(result: RmiCommandReturnCode, pdev_ptr: Address, coh: RmiPdevCoherent, old_s: S, new_s: S) -> bool {
    (!ImplFeatures().feat_da == FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((coh == RMI_NCOH && PdevAt(old_s, pdev_ptr).ncoh_ide != IDE_TRUE) || (coh == RMI_COH && PdevAt(old_s, pdev_ptr).coh_ide != IDE_TRUE)) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(old_s, pdev_ptr).state != PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && ((ImplFeatures().feat_da == FEATURE_TRUE && AddrIsGranuleAligned(pdev_ptr) && PaIsDelegable(pdev_ptr) && GranuleAt(old_s, pdev_ptr).state == PDEV && ((coh == RMI_NCOH && PdevAt(old_s, pdev_ptr).ncoh_ide == IDE_TRUE) || (coh == RMI_COH && PdevAt(old_s, pdev_ptr).coh_ide == IDE_TRUE)) && PdevAt(old_s, pdev_ptr).state == PDEV_READY) ==> (result == RMI_SUCCESS && PdevAt(new_s, pdev_ptr).state == PDEV_COMMUNICATING && PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING))
}