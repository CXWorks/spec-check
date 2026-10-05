pub open spec fn base_discover_vendor__3_2_2_5_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> IsAsciiString(new_s.vendor_identifier))
    && (result != RSI_SUCCESS ==> true)
    && (old_s == new_s)
}