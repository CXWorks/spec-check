pub open spec fn ffa_rx_release_spec(result: int32, old_s: S, new_s: S) -> bool {
    (old_s.ffa_instance == FFA_INSTANCE_NON_SECURE_PHYSICAL ==> (old_s.vm_id == (result as int32) as u32))
    && (old_s.ffa_instance != FFA_INSTANCE_NON_SECURE_PHYSICAL ==> result == FFA_ERROR_INVALID_PARAMETERS)
    && (old_s.ffa_instance == FFA_INSTANCE_NON_SECURE_PHYSICAL ==> (old_s.vm_id == (result as int32) as u32 ==> result == FFA_SUCCESS))
    && (old_s.ffa_instance == FFA_INSTANCE_NON_SECURE_PHYSICAL ==> (old_s.vm_id != (result as int32) as u32 ==> result == FFA_ERROR_DENIED))
    && (old_s.ffa_instance == FFA_INSTANCE_NON_SECURE_PHYSICAL ==> (old_s.vm_id == (result as int32) as u32 ==> new_s.vm_id == old_s.vm_id))
    && (old_s.ffa_instance != FFA_INSTANCE_NON_SECURE_PHYSICAL ==> result == FFA_ERROR_INVALID_PARAMETERS)
}