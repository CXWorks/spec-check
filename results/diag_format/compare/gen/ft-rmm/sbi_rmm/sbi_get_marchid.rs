pub open spec fn sbi_get_marchid_spec(value: struct sbiret, old_s: S, new_s: S) -> bool {
  IsLegalMarchidValue(value)
}