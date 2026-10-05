pub open spec fn sbi_sse_inject_spec(event_id: UInt32, hart_id: UInt64, result: SbiCommandReturnCode, old_s: S, new_s: S) -> bool {
  (result == SBI_SUCCESS)
}