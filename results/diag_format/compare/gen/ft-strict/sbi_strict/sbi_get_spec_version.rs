pub open spec fn sbi_get_spec_version_spec(value: UInt, error: Int, old_s: S, new_s: S) -> bool {
  (CallSucceeded(error))
  && (Bits(value, 23, 0) == SbiSpecMinorVersion())
  && (Bits(value, 30, 24) == SbiSpecMajorVersion())
  && (Bits(value, 31, 31) == 0)
  && (XLEN > 32 ==> Bits(value, XLEN - 1, 32) == 0)
}