pub open spec fn rmi_pdev_ide_reset_spec(pdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    let pdev_ptr_is_valid_fid = (0xC4000179u64 == 0xC4000179u64);
    let pdev_ptr_is_aligned = AddrIsGranuleAligned(old_s, pdev_ptr);
    let pdev_ptr_is_delegable = PaIsDelegable(old_s, pdev_ptr);
    let pdev_ptr_is_pdev = GranuleAt(old_s, pdev_ptr).state == PDEV;
    let pdev = PdevAt(old_s, pdev_ptr);
    let pdev_ncoh_ide_is_true = pdev.ncoh_ide == IDE_TRUE;
    let pdev_state_is_ready = pdev.state == PDEV_READY;
    let pdev_state_is_resetting = pdev.state == PDEV_IDE_RESETTING;
    let pdev_comm_state_is_pending = pdev.comm_state == DEV_COMM_PENDING;
    (!pdev_ptr_is_valid_fid ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (pdev_ptr_is_valid_fid && !pdev_ptr_is_aligned ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev_ptr_is_valid_fid && pdev_ptr_is_aligned && !pdev_ptr_is_delegable ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev_ptr_is_valid_fid && pdev_ptr_is_aligned && pdev_ptr_is_delegable && !pdev_ptr_is_pdev ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (pdev_ptr_is_valid_fid && pdev_ptr_is_aligned && pdev_ptr_is_delegable && pdev_ptr_is_pdev && !pdev_ncoh_ide_is_true ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (pdev_ptr_is_valid_fid && pdev_ptr_is_aligned && pdev_ptr_is_delegable && pdev_ptr_is_pdev && pdev_ncoh_ide_is_true && !pdev_state_is_ready ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (pdev_ptr_is_valid_fid && pdev_ptr_is_aligned && pdev_ptr_is_delegable && pdev_ptr_is_pdev && pdev_ncoh_ide_is_true && pdev_state_is_ready ==> result.is_Ok() && pdev_state_is_resetting && pdev_comm_state_is_pending)
}