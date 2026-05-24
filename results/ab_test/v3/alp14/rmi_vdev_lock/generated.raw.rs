pub open spec fn rmi_vdev_lock_spec(result: RmiCommandReturnCode, rd: Address, vdev_ptr: Address, old_s: S, new_s: S) -> bool {
    let realm = RealmAt(rd);
    let vdev = VdevAt(vdev_ptr);
    
    (!ImplFeatures().feat_da_is_true() ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    && (!AddrIsGranuleAligned(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!AddrIsGranuleAligned(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (!PaIsDelegable(vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (vdev.vdev_state != VDEV_UNLOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
    && (
        (AddrIsGranuleAligned(rd)
         && PaIsDelegable(rd)
         && GranuleAt(old_s, rd).state == RD
         && AddrIsGranuleAligned(vdev_ptr)
         && PaIsDelegable(vdev_ptr)
         && GranuleAt(old_s, vdev_ptr).state == VDEV
         && vdev.realm == rd
         && vdev.vdev_state == VDEV_UNLOCKED
         && vdev.comm_state == DEV_COMM_IDLE
         && ImplFeatures().feat_da_is_true())
        ==> (result.is_Ok()
             && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK
             && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
    )
}