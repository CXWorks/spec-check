pub open spec fn sbi_get_mvendorid_spec(value: UInt64, old_s: S, new_s: S) -> bool {
  (IsLegalMvendoridValue(value))
}