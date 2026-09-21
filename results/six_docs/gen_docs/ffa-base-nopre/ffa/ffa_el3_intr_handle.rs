pub open spec fn ffa_el3_intr_handle_spec(result: int32, old_s: S, new_s: S) -> bool {
    (result == NOT_SUPPORTED ==> (old_s.ff_a_instance != FF_A_INSTANCE_SECURE_PHYSICAL || old_s.scr_el3_fiq == 1))
    && (result == FFA_SUCCESS ==> (old_s.ff_a_instance == FF_A_INSTANCE_SECURE_PHYSICAL && old_s.scr_el3_fiq == 0))
    && (result == FFA_SUCCESS ==> new_s == old_s)
}