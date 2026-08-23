pub open spec fn rmi_mec_set_shared_spec(mecid: Bits64, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    ((mecid as int) > (ImplFeatures(old_s).max_mecid as int) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (MecState(old_s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (((mecid as int) <= (ImplFeatures(old_s).max_mecid as int)
        && MecState(old_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED)
        ==> result.is_Ok() && MecState(new_s, mecid) == MEC_STATE_SHARED)
}