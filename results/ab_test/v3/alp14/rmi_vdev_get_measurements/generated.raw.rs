pub open spec fn rmi_vdev_get_measurements_spec(result: RmiCommandReturnCode, rd: Address, vdev_ptr: Address, params_ptr: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(old_s, rd);
    let vdev = VdevAt(old_s, vdev_ptr);
    let params = RmiVdevMeasureParamsAt(old_s, params_ptr);
    
    (!ImplFeatures(old_s).feat_da ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((vdev.vdev_state != VDEV_LOCKED && vdev.vdev_state != VDEV_STARTED) ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (!AddrIsGranuleAligned(old_s, params_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!GranuleAccessPermitted(old_s, params_ptr, PAS_NS) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((params.indices[0] == 1 || params.indices[255] == 1) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((ImplFeatures(old_s).feat_da && AddrIsGranuleAligned(old_s, rd) && PaIsDelegable(old_s, rd) && GranuleAt(old_s, rd).state == RD
         && AddrIsGranuleAligned(old_s, vdev_ptr) && PaIsDelegable(old_s, vdev_ptr) && GranuleAt(old_s, vdev_ptr).state == VDEV
         && vdev.realm == rd && (vdev.vdev_state == VDEV_LOCKED || vdev.vdev_state == VDEV_STARTED)
         && vdev.comm_state == DEV_COMM_IDLE && AddrIsGranuleAligned(old_s, params_ptr)
         && GranuleAccessPermitted(old_s, params_ptr, PAS_NS) && params.indices[0] != 1 && params.indices[255] != 1)
        ==> (result.is_Ok() && VdevAt(new_s, vdev_ptr).op == VDEV_OP_GET_MEAS && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING))
}