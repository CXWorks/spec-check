pub open spec fn RMI_VDEV_COMPLETE_spec(s: S, rec_ptr: Address, vdev_ptr: Address) -> (result: RmiCommandReturnCode, s_prime: S)
{
    let rec = RecAt(s, rec_ptr);
    let vdev = VdevAt(s, vdev_ptr);
    
    // Failure conditions
    if !AddrIsGranuleAligned(rec_ptr) {
        (RMI_ERROR_INPUT, s)
    } else if !PaIsDelegable(rec_ptr) {
        (RMI_ERROR_INPUT, s)
    } else if GranuleAt(s, rec_ptr).state != REC {
        (RMI_ERROR_INPUT, s)
    } else if !AddrIsGranuleAligned(vdev_ptr) {
        (RMI_ERROR_INPUT, s)
    } else if !PaIsDelegable(vdev_ptr) {
        (RMI_ERROR_INPUT, s)
    } else if GranuleAt(s, vdev_ptr).state != VDEV {
        (RMI_ERROR_INPUT, s)
    } else if rec.pending != REC_PENDING_VDEV_REQUEST {
        (RMI_ERROR_INPUT, s)
    } else if rec.owner != vdev.realm {
        (RMI_ERROR_INPUT, s)
    } else if rec.vdev_id_1 != vdev.vdev_id {
        (RMI_ERROR_INPUT, s)
    } else if vdev.comm_state != DEV_COMM_IDLE {
        (RMI_ERROR_DEVICE, s)
    } else {
        // Success: update rec and vdev
        let rec_prime = rec.pending := REC_PENDING_VDEV_COMPLETE;
        let rec_prime = rec_prime.vdev_pa_1 := vdev_ptr;
        let vdev_prime = vdev.comm_state := DEV_COMM_PENDING;
        
        let s_prime = s.update_rec(rec_ptr, rec_prime).update_vdev(vdev_ptr, vdev_prime);
        
        (RMI_SUCCESS, s_prime)
    }
}