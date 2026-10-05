pub open spec fn mem_protect_check_range_spec(result: PsciReturnCode, base: UInt64, length: UInt64, old_s: S, new_s: S) -> bool {
    (!IsFunctionImplemented(MEM_PROTECT_CHECK_RANGE) ==> ResultEqual(result, NOT_SUPPORTED))
    && ((IsFunctionImplemented(MEM_PROTECT_CHECK_RANGE) && !IsRangeProtectedByMemProtect(base, base + length - 1)) ==> ResultEqual(result, DENIED))
    && ((IsFunctionImplemented(MEM_PROTECT_CHECK_RANGE) && IsRangeProtectedByMemProtect(base, base + length - 1)) ==> (ResultEqual(result, SUCCESS) && new_s == old_s))
    && (new_s == old_s)
}
