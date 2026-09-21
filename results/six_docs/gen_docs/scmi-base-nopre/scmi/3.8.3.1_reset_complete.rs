pub open spec fn 3.8.3.1_reset_complete_spec(result: int32, domain_id: uint32, old_s: S, new_s: S) -> bool {
    (result == 0x16_00000004 ==> true)
    && (result == 0x16_00000004 ==> domain_id == domain_id)
    && (result == 0x16_00000004 ==> old_s == new_s)
    && (result == 0x16_00000004 ==> true)
}