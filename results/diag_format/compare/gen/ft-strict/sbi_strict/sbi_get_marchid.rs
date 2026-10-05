pub open spec fn sbi_get_marchid_spec(value: UInt64, old_s: S, new_s: S) -> bool {
  IsLegalMarchidValue(value)
  && IsLegalMarchidValue(0)
  && true
}