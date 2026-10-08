pub open spec fn rmi_pdev_ide_reset_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (ImplFeatures(old_s).feat_da != RmmFeature::FEATURE_TRUE ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, pdev_ptr).state != RmmGranuleState::PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (PdevAt(old_s, pdev_ptr).ncoh_ide != RmmPdevIde::IDE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (PdevAt(old_s, pdev_ptr).state != RmmPdevState::PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (result.is_Ok() ==> (PdevAt(new_s, pdev_ptr).state == RmmPdevState::PDEV_IDE_RESETTING && PdevAt(new_s, pdev_ptr).comm_state == RmmDevCommState::DEV_COMM_PENDING))
}