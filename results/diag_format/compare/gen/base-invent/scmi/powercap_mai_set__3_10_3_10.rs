pub open spec fn powercap_mai_set__3_10_3_10_spec(result: int32, old_s: S, new_s: S) -> bool {
    (flags_not_zero(old_s.flags) ==> ResultEqual(result, NOT_FOUND))
    && (flags_not_zero(old_s.flags) ==> ResultEqual(result, NOT_SUPPORTED))
    && (flags_not_zero(old_s.flags) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (flags_not_zero(old_s.flags) ==> ResultEqual(result, DENIED))
    && (flags_not_zero(old_s.flags) ==> ResultEqual(result, SUCCESS))
}

fn flags_not_zero(flags: uint32) -> bool {
    flags != 0
}