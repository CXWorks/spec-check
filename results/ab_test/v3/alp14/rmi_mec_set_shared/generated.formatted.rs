pub open spec fn rmi_mec_set_shared_spec(result: Result<(), RmiStatusCode>, mecid: u64, old_s: S, new_s: S) -> bool {
    let max_mecid = ImplFeatures().max_mecid;
    (((mecid as int) > (max_mecid as int)) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((MecState(old_s, mecid) != MEC_STATE_PRIVATE_UNASSIGNED) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((result.is_Ok() && MecState(old_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED) ==> (MecState(new_s, mecid) == MEC_STATE_SHARED))
}