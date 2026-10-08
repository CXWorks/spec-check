pub open spec fn rmi_pdev_ide_reset_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!GranuleAccessPermitted(old_s, pdev_ptr, PAS_REALM) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, pdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAt(old_s, pdev_ptr).state == PDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PdevAt(old_s, pdev_ptr).ncoh_ide == IDE_TRUE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!PdevAt(old_s, pdev_ptr).state == PDEV_READY ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (ResultEqual(result, RMI_SUCCESS) ==> PdevAt(new_s, pdev_ptr).state == PDEV_IDE_RESETTING)
    && (ResultEqual(result, RMI_SUCCESS) ==> PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING)
    && (ResultEqual(result, RMI_SUCCESS) ==> PdevAt(new_s, pdev_ptr).ncoh_ide == IDE_TRUE)
    && (ResultEqual(result, RMI_SUCCESS) ==> PdevAt(new_s, pdev_ptr).state == PDEV_READY)
    && (ResultEqual(result, RMI_SUCCESS) ==> PdevAt(new_s, pdev_ptr).state == PDEV_IDE_RESETTING)
}