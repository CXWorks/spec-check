pub open spec fn protocol_version__3_10_3_1_spec(result: RsiCommandReturnCode, version: u32, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> version == 0x30000)
    && (result != RSI_SUCCESS ==> true)
    && (old_s == new_s)
}