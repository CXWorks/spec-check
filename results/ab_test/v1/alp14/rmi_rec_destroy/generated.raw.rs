```verus
pub open spec fn RMI_REC_DESTROY_spec(
    s: S,
    rec_ptr: Address,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let rd_pre = RecAt(s, rec_ptr).owner;
    let realm_pre = RealmAt(s, rd_pre);
    let rec_pre = RecAt(s, rec_ptr);
    
    // Failure conditions
    let rec_align_fail = !AddrIsGranuleAligned(rec_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
    let rec_bound_fail = !PaIsDelegable(rec_ptr) && ResultEqual(result, RMI_ERROR_INPUT);
    let rec_gran_state_fail = GranuleAt(s, rec_ptr).state != REC && ResultEqual(result, RMI_ERROR_INPUT);
    let rec_state_fail = rec_pre.state == REC_RUNNING && ResultEqual(result, RMI_ERROR_REC);
    
    // Success conditions (when no failure)
    let success = result.is_Ok() && {
        let realm = RealmAt(s, rd_pre);
        let rec = RecAt(s, rec_ptr);
        
        GranuleAt(s, rec_ptr).state == DELEGATED &&
        AuxStateEqual32(rec_pre.aux, RecAuxCount(rd_pre), DELEGATED) &&
        realm.num_recs == realm_pre.num_recs - 1
    };
    
    // Failure condition ordering:
    // [rec_bound, rec_gran_state] < [rec_state]
    // rec_align < rec_gran_state < rec_bound
    
    if rec_align_fail {
        rec_align_fail
    } else if rec_gran_state_fail {
        rec_gran_state_fail
    } else if rec_bound_fail {
        rec_bound_fail
    } else if rec_state_fail {
        rec_state_fail
    } else {
        success
    }
}
```