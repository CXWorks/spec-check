```verus
pub open spec fn RMI_MEC_SET_SHARED_spec(old_s: S, new_s: S, mecid: u64, result: Result<(), RmiStatusCode>) -> bool {
    let max_mecid = ImplFeatures().max_mecid;
    let old_mec_state = MecState(old_s, mecid);
    let new_mec_state = MecState(new_s, mecid);
    
    // Failure condition: mecid_bound
    (UInt(mecid) > UInt(max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Failure condition: state
    (old_mec_state != MEC_STATE_PRIVATE_UNASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
    
    // Success condition: mec_state (only when no failure conditions met)
    ((UInt(mecid) <= UInt(max_mecid) && old_mec_state == MEC_STATE_PRIVATE_UNASSIGNED) ==>
     (result.is_Ok() && new_mec_state == MEC_STATE_SHARED))
}
```