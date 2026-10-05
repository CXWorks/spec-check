pub open spec fn mem_protect_check_range_spec(base: Address, length: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsImplemented(old_s, MEM_PROTECT_CHECK_RANGE) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!RangeIsProtectedByMemProtect(old_s, base, base + length - 1) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> RangeIsProtectedByMemProtect(new_s, base, base + length - 1))
  && (result == SUCCESS ==> IsImplemented(new_s, MEM_PROTECT))
  && ((IsImplemented(old_s, MEM_PROTECT_CHECK_RANGE) &&
       RangeIsProtectedByMemProtect(old_s, base, base + length - 1))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> RangeIsProtectedByMemProtect(new_s, base, base + length - 1))
  && (result != SUCCESS
    ==> IsImplemented(new_s, MEM_PROTECT))
  && (result != DENIED && result != NOT_SUPPORTED
    ==> RangeIsProtectedByMemProtect(new_s, base, base + length - 1))
  && (result != DENIED && result != NOT_SUPPORTED
    ==> IsImplemented(new_s, MEM_PROTECT))
}