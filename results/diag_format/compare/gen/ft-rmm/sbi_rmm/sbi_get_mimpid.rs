pub open spec fn sbi_get_mimpid_spec(value: struct sbiret, old_s: S, new_s: S) -> bool {
  IsLegalMimpidValue(value)
}