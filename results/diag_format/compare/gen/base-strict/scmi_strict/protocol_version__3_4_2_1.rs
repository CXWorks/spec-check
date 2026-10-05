pub open spec fn protocol_version__3_4_2_1_spec(result: RsiCommandReturnCode, version: UInt32, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (version == 0x20001)
    && (old_s == new_s)
}