```verus
pub open spec fn RMI_MEC_SET_SHARED_spec(s: S, mecid: u64, result: Result<(), RmiStatusCode>) -> bool {
    if UInt64(mecid) > UInt64(ImplFeatures(s).max_mecid) {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else if MecState(s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED {
        ResultEqual(result, RMI_ERROR_INPUT)
    } else {
        result.is_Ok() && MecState(s, mecid) == MEC_STATE_SHARED
    }
}
```