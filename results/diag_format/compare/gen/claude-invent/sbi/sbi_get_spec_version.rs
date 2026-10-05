pub open spec fn sbi_get_spec_version_spec(error: i64, value: u64, old_s: S, new_s: S) -> bool {
    (error == 0)
    && ((value & 0xFF_FFFFu64) as int == SbiSpecVersionMinor(old_s))
    && (((value >> 24u64) & 0x7Fu64) as int == SbiSpecVersionMajor(old_s))
    && ((value >> 31u64) == 0u64)
    && (new_s == old_s)
}
