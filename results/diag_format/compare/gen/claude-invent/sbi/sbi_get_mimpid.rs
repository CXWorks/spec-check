pub open spec fn sbi_get_mimpid_spec(value: UInt64, old_s: S, new_s: S) -> bool {
    IsLegalMimpidCsrValue(old_s, value)
    && new_s == old_s
}
