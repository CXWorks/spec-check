pub open spec fn sbi_system_reset_spec(reset_type: UInt32, reset_reason: UInt32, result: SbiRet, old_s: S, new_s: S) -> bool {
    ((((reset_type as int) >= 0x0000_0003 && (reset_type as int) <= 0xEFFF_FFFF)
        || ((reset_reason as int) >= 0x0000_0002 && (reset_reason as int) <= 0xDFFF_FFFF))
        ==> (result.error == SBI_ERR_INVALID_PARAM && new_s == old_s))
    && (result.error != SBI_SUCCESS)
    && ((result.error != SBI_SUCCESS) ==> new_s == old_s)
}
