pub open spec fn protocol_version__3_12_4_1_spec(result: RsiCommandReturnCode, version: u32, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (version == 0x10000)
    && (old_s == new_s)
}