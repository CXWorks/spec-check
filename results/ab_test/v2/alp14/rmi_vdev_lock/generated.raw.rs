```verus
pub open spec fn rmi_vdev_lock_spec(result: RmiCommandReturnCode, old_s: S, new_s: S) -> bool {
    let rd = old_s.input_x1;
    let vdev_ptr = old_s.input_x2;
    
    // Failure condition: da_supp
    && (!ImplFeatures(old_s).feat_da_eq_feature_true(old_s) ==> ResultEqual(result, RMI_ERROR_NOT_SUPPORTED))
    
    // Failure condition: rd_align
    && (!AddrIsGranuleAligned(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: rd_bound
    && (!PaIsDelegable(old_s, rd) ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: rd_state
    && (GranuleAt(old_s, rd).state != RD ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: vdev_align
    && (!AddrIsGranuleAligned(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: vdev_bound
    && (!PaIsDelegable(old_s, vdev_ptr) ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Failure condition: vdev_gran_state
    && (GranuleAt(old_s, vdev_ptr).state != VDEV ==> ResultEqual(result, RMI_ERROR_INPUT))
    
    // Context extraction with ordering constraints
    && (let realm = RealmAt(old_s, rd);
        let vdev = VdevAt(old_s, vdev_ptr);
        
        // Failure condition: vdev_realm (ordered after vdev_gran_state)
        && (vdev.realm != rd ==> ResultEqual(result, RMI_ERROR_INPUT))
        
        // Failure condition: vdev_state (ordered after vdev_gran_state)
        && (vdev.vdev_state != VDEV_UNLOCKED ==> ResultEqual(result, RMI_ERROR_DEVICE))
        
        // Failure condition: comm_state (ordered after vdev_gran_state)
        && (vdev.comm_state != DEV_COMM_IDLE ==> ResultEqual(result, RMI_ERROR_DEVICE))
        
        // Success conditions (when all preconditions are met)
        && (ImplFeatures(old_s).feat_da_eq_feature_true(old_s)
            && AddrIsGranuleAligned(old_s, rd)
            && PaIsDelegable(old_s, rd)
            && GranuleAt(old_s, rd).state == RD
            && AddrIsGranuleAligned(old_s, vdev_ptr)
            && PaIsDelegable(old_s, vdev_ptr)
            && GranuleAt(old_s, vdev_ptr).state == VDEV
            && vdev.realm == rd
            && vdev.vdev_state == VDEV_UNLOCKED
            && vdev.comm_state == DEV_COMM_IDLE
            ==> (result == RMI_SUCCESS
                 && VdevAt(new_s, vdev_ptr).op == VDEV_OP_LOCK
                 && VdevAt(new_s, vdev_ptr).comm_state == DEV_COMM_PENDING)
        )
    )
}
```