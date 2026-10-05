pub open spec fn sbi_get_marchid_spec(value: UInt64, old_s: S, new_s: S) -> bool {
    (value == 0 || IsLegalMarchidValue(old_s, value))
    && new_s == old_s
}
