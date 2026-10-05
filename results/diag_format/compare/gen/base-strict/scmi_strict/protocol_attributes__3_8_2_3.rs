pub open spec fn protocol_attributes_spec(result: RsiCommandReturnCode, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (true ==> result == RSI_SUCCESS)
    && (true ==> Bits(attributes, 31, 16) == 0)
    && (true ==> Bits(attributes, 15, 0) == NumResetDomains())
    && (true ==> old_s == new_s)
}