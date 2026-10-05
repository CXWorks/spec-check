pub open spec fn mem_protect_check_range_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(MEM_PROTECT_CHECK_RANGE) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsRangeProtectedByMemProtect(old_s, base, base + length - 1) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> IsRangeProtectedByMemProtect(old_s, base, base + length - 1))
    && (old_s == new_s)
}