pub open spec fn system_suspend_spec(entry_point_address: UInt64, context_id: UInt64, result: Result<(), PsciStatusCode>, old_s: S, new_s: S) -> bool {
  (result == NOT_SUPPORTED ==> true)
  && (result == INVALID_ADDRESS ==> true)
  && (result == DENIED ==> true)
  && (result == SUCCESS ==> true)
}