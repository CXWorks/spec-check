pub open spec fn sbi_get_mimpid_spec(value: UInt, old_s: S, new_s: S) -> bool {
  IsLegalMimpidValue(value)
  && IsLegalMimpidValue(0)
}