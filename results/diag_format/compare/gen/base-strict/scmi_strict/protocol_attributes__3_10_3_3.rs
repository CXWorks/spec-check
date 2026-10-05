pub open spec fn protocol_attributes__3_10_3_3_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS ==> (Bits64(new_s.payload.attributes, 31, 16) == 0u64))
    && (result == RSI_SUCCESS ==> (Bits64(new_s.payload.attributes, 15, 0) == NumPowerCappingDomains()))
    && (result != RSI_SUCCESS ==> true)
}