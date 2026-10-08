pub open spec fn rmi_vdev_start_spec(rd: Address, vdev_ptr: Address, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!VdevAt(old_s, vdev_ptr).realm == RealmAt(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (VdevAt(old_s, vdev_ptr).state != VDEV_LOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (VdevAt(old_s, vdev_ptr).comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (result.is_Ok() ==> (VdevAt(new_s, vdev_ptr).op == VDEV_OP_START && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING))
}