pub open spec fn mem_protect_check_range_spec(base: UInt64, length: UInt64, result: Result<(), PsciStatusCode>, old_s: S, new_s: S) -> bool {
  (result == NOT_SUPPORTED ==> (base == 0 && length == 0))
}