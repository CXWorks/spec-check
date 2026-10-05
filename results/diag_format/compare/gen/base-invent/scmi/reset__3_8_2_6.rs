pub open spec fn reset__3_8_2_6_spec(result: int32, old_s: S, new_s: S) -> bool {
    ((old_s.flags & 0x7) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((old_s.flags & 0x4) != 0 && (old_s.flags & 0x1) == 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((old_s.flags & 0x2) != 0 && (old_s.flags & 0x1) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((old_s.domain_id as int) < 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((old_s.reset_state as int) < 0 ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((old_s.reset_state as int) >= 0x100 ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((old_s.flags & 0x7) == 0 && (old_s.domain_id as int) >= 0 && (old_s.reset_state as int) >= 0 && (old_s.reset_state as int) < 0x100 ==> (result == SUCCESS || result == NOT_FOUND || result == GENERIC_ERROR || result == DENIED))
}