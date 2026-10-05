pub open spec fn sbi_get_spec_version_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (result.minor == CurrentSbiSpecMinorVersion())
    && (result.major == CurrentSbiSpecMajorVersion())
    && (result.reserved == 0)
    && ((XLEN > 32) ==> (result.reserved_hi == 0))
}