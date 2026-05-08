```verus
pub open spec fn RMI_MEC_SET_PRIVATE_spec(
    old_s: S,
    new_s: S,
    mecid: u64,
    result: Result<(), RmiStatusCode>,
) -> bool {
    (
        // Failure condition: mecid_bound
        (UInt(mecid) > UInt(ImplFeatures().max_mecid) ==>
            ResultEqual(result, RMI_ERROR_INPUT))
    ) && (
        // Failure condition: state
        (MecState(old_s, mecid) != MEC_STATE_SHARED ==>
            ResultEqual(result, RMI_ERROR_INPUT))
    ) && (
        // Failure condition: members
        (MecMembers(old_s, mecid) != 0 ==>
            ResultEqual(result, RMI_ERROR_INPUT))
    ) && (
        // Success condition: mec_state
        (!ResultEqual(result, RMI_ERROR_INPUT) && 
         UInt(mecid) <= UInt(ImplFeatures().max_mecid) &&
         MecState(old_s, mecid) == MEC_STATE_SHARED &&
         MecMembers(old_s, mecid) == 0) ==>
            (result.is_Ok() && MecState(new_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
    )
}
```