pub open spec fn mem_protect_check_range_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!IsImplemented(MEM_PROTECT_CHECK_RANGE) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!RangeIsProtectedByMemProtect(old_s, base, base + length - 1) ==> ResultEqual(result, DENIED))
    && (ResultEqual(result, SUCCESS) ==> RangeIsProtectedByMemProtect(old_s, base, base + length - 1))
    && (ResultEqual(result, SUCCESS) ==> IsImplemented(MEM_PROTECT))
    && (old_s == new_s)
}