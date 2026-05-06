pub open spec fn RMI_PDEV_IDE_RESET_spec(old_s: S, new_s: S, pdev_ptr: Address, result: Result<(), RmiStatusCode>) -> bool {
    ((!ImplFeatures(old_s).feat_da) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED())) &&
    ((!AddrIsGranuleAligned(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
    ((!PaIsDelegable(pdev_ptr)) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
    ((GranuleAt(old_s, pdev_ptr).state != PDEV()) ==> ResultEqual(result, RMI_ERROR_INPUT())) &&
    ((PdevAt(old_s, pdev_ptr).ncoh_ide != IDE_TRUE()) ==> ResultEqual(result, RMI_ERROR_DEVICE())) &&
    ((PdevAt(old_s, pdev_ptr).state != PDEV_READY()) ==> ResultEqual(result, RMI_ERROR_DEVICE())) &&
    (result.is_Ok() ==> (
        PdevAt(new_s, pdev_ptr).state == PDEV_IDE_RESETTING() &&
        PdevAt(new_s, pdev_ptr).comm_state == DEV_COMM_PENDING()
    ))
}