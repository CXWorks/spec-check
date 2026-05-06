pub open spec fn RMI_MEC_SET_PRIVATE_spec(
    old_s: S,
    new_s: S,
    mecid: Bits64,
    result: Result<(), RmiStatusCode>,
) -> bool {
    let mecid_val = mecid as int;
    let max_mecid = ImplFeatures(old_s).max_mecid as int;
    let mec_state_old = MecState(old_s, mecid);
    let mec_members_old = MecMembers(old_s, mecid);

    (
    // mecid_bound failure
    (mecid_val > max_mecid ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // state failure
    (mec_state_old != MEC_STATE_SHARED ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // members failure
    (mec_members_old != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
        &&
    // Success condition
    ((mecid_val <= max_mecid && mec_state_old == MEC_STATE_SHARED && mec_members_old == 0) ==> (
    result.is_Ok() && MecState(new_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED)))
}