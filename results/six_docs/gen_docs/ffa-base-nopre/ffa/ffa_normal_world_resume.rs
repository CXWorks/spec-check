pub open spec fn ffa_normal_world_resume_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == 0x80000002 ==> (old_s.ffa_instance == FfaInstance::SecurePhysical && !old_s.normal_world_preempted))
    && (result == 0x80000003 ==> (old_s.ffa_instance != FfaInstance::SecurePhysical))
    && (result == 0x80000000 ==> (old_s.ffa_instance == FfaInstance::SecurePhysical && old_s.normal_world_preempted))
    && (result != 0x80000000 && result != 0x80000002 && result != 0x80000003 ==> true)
}