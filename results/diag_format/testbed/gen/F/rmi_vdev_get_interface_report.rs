pub open spec fn rmi_vdev_get_interface_report_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!RmmGranuleAccessPermitted(old_s, rd, PAS_REALM) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmmGranuleState(old_s, rd).state == RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmmGranuleAccessPermitted(old_s, vdev_ptr, PAS_REALM) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmmGranuleState(old_s, vdev_ptr).state == VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!RmmVdevState(old_s, vdev_ptr).state == VDEV_LOCKED && !RmmVdevState(old_s, vdev_ptr).state == VDEV_STARTED ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!RmmVdevCommState(old_s, vdev_ptr) == DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (RmmVdevOp(old_s, vdev_ptr) == VDEV_OP_GET_REPORT && RmmVdevCommState(old_s, vdev_ptr) == DEV_COMM_PENDING ==> result.is_Ok() && RmmVdevOp(new_s, vdev_ptr) == VDEV_OP_GET_REPORT && RmmVdevCommState(new_s, vdev_ptr) == DEV_COMM_PENDING)
}