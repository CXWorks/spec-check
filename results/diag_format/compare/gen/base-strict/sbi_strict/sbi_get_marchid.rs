pub open spec fn sbi_get_marchid_spec(result: Int64, value: UInt64, old_s: S, new_s: S) -> bool {
    (true ==> result == 0)
    && (true ==> IsLegalMarchidValue(value))
    && (true ==> IsLegalMarchidValue(0))
    && (true ==> old_s == new_s)
}