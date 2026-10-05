pub open spec fn ffa_normal_world_resume_spec(result: int32, old_s: S, new_s: S) -> bool {
    (old_s.ffa_instance == FF_A_SECURE_PHYSICAL ==> (
        (!old_s.normal_world_preempted ==> ResultEqual(result, FFA_ERROR_DENIED))
        && (old_s.normal_world_preempted ==> true)
    ))
    && (old_s.ffa_instance != FF_A_SECURE_PHYSICAL ==> ResultEqual(result, FFA_ERROR_NOT_SUPPORTED))
}