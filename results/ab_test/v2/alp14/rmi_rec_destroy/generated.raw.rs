```verus
pub open spec fn rmi_rec_destroy_spec(result: RmiCommandReturnCode, rec_ptr: Address, old_s: S, new_s: S) -> bool {
    // Failure condition: rec_align
    ((!AddrIsGranuleAligned(old_s, rec_ptr)) ==> result == RMI_ERROR_INPUT) &&
    
    // Failure condition: rec_bound
    ((!PaIsDelegable(old_s, rec_ptr)) ==> result == RMI_ERROR_INPUT) &&
    
    // Failure condition: rec_gran_state (checking before rec_state due to ordering)
    ((GranuleAt(old_s, rec_ptr).state != REC) ==> result == RMI_ERROR_INPUT) &&
    
    // Failure condition: rec_state
    ((RecAt(old_s, rec_ptr).state == REC_RUNNING) ==> result == RMI_ERROR_REC) &&
    
    // Success conditions (when all preconditions are met)
    (
        (AddrIsGranuleAligned(old_s, rec_ptr) &&
         PaIsDelegable(old_s, rec_ptr) &&
         GranuleAt(old_s, rec_ptr).state == REC &&
         RecAt(old_s, rec_ptr).state != REC_RUNNING)
        ==>
        (result == RMI_SUCCESS &&
         GranuleAt(new_s, rec_ptr).state == DELEGATED &&
         AuxStateEqual32(RecAt(old_s, rec_ptr).aux, 
                        RecAuxCount(RecAt(old_s, rec_ptr).owner), 
                        DELEGATED) &&
         RealmAt(new_s, RecAt(old_s, rec_ptr).owner).num_recs == 
         RealmAt(old_s, RecAt(old_s, rec_ptr).owner).num_recs - 1)
    )
}
```