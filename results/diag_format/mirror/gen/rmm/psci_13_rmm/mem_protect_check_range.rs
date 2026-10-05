pub open spec fn mem_protect_check_range_spec(base: Address, length: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
  (!IsFunctionImplemented(old_s, MEM_PROTECT_CHECK_RANGE) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsRangeProtectedByMemProtect(old_s, base, base + length - 1) ==> ResultEqual(result, DENIED))
  && (result == SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == SUCCESS ==> IsRangeProtectedByMemProtect(new_s, base, base + length - 1))
  && ((IsFunctionImplemented(old_s, MEM_PROTECT_CHECK_RANGE) &&
       IsRangeProtectedByMemProtect(old_s, base, base + length - 1))
    ==> result == SUCCESS)
  && (result != SUCCESS
    ==> IsRangeProtectedByMemProtect(new_s, base, base + length - 1))
}