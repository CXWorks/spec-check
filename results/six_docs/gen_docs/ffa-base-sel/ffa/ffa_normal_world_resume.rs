pub open spec fn ffa_normal_world_resume_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == FFA_DENIED ==> old_s == new_s)
    && (result == FFA_NOT_SUPPORTED ==> old_s == new_s)
    && (result != FFA_DENIED && result != FFA_NOT_SUPPORTED ==> old_s == new_s)
}