pub open spec fn rmi_mec_set_shared_spec(
    result: RmiCommandReturnCode,
    mecid: u64,
    old_s: S,
    new_s: S,
) -> bool {
    // Failure condition: mecid_bound
    (UInt(mecid) > UInt(ImplFeatures().max_mecid) ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Failure condition: state
     && (MecState(old_s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED ==> ResultEqual(
        result,
        RMI_ERROR_INPUT,
    ))
    // Success condition: mec_state
     && (UInt(mecid) <= UInt(ImplFeatures().max_mecid) && MecState(old_s, mecid)
        == MEC_STATE_PRIVATE_UNASSIGNED ==> result.is_Ok() && MecState(new_s, mecid)
        == MEC_STATE_SHARED)
}