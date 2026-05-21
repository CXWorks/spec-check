pub open spec fn rmi_mec_set_private_spec(result: RmiCommandReturnCode, mecid: u64, old_s: S, new_s: S) -> bool {
    (UInt(mecid) > UInt(ImplFeatures().max_mecid) ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (MecState(old_s, mecid) != MEC_STATE_SHARED ==> ResultEqual(result, RMI_ERROR_INPUT))
    && (MecMembers(old_s, mecid) != 0 ==> ResultEqual(result, RMI_ERROR_INPUT))
    && ((UInt(mecid) <= UInt(ImplFeatures().max_mecid)
        && MecState(old_s, mecid) == MEC_STATE_SHARED
        && MecMembers(old_s, mecid) == 0)
        ==> (!result.is_Err() && MecState(new_s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED))
}