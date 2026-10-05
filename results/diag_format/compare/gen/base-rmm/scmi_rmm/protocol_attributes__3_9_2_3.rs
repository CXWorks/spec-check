pub open spec fn protocol_attributes__3_9_2_3_spec(result: RsiCommandReturnCode, attributes: u64, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (attributes & 0xFFFF_0000 == 0)
    && (attributes & 0xFFFF == NumVoltageDomains())
    && (old_s == new_s)
}