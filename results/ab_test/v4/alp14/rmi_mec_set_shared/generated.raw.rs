pub open spec fn RMI_MEC_SET_SHARED_spec(old_s: S, new_s: S, mecid: u64, result: Result<(), RmiStatusCode>) -> bool {
    (
        (UInt(mecid) > UInt(ImplFeatures().max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT)) &&
        (MecState(old_s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
    ) || (
        result.is_Ok() &&
        MecState(new_s, mecid) == MEC_STATE_SHARED &&
        UInt(mecid) <= UInt(ImplFeatures().max_mecid) &&
        MecState(old_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED
    )
}