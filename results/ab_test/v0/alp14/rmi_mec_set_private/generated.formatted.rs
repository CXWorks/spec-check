pub open spec fn RMI_MEC_SET_PRIVATE_spec(s: S, mecid: u64) -> bool {
    let max_mecid = ImplFeatures(s).max_mecid;
    let mec_state = MecState(s, mecid);
    let mec_members = MecMembers(s, mecid);

    if (mecid as int) > (max_mecid as int) {
        false
    } else if mec_state != MEC_STATE_SHARED {
        false
    } else if mec_members != 0 {
        false
    } else {
        MecState(s, mecid) == MEC_STATE_PRIVATE_UNASSIGNED
    }
}