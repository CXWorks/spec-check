pub open spec fn protocol_attributes_spec(result: RsiCommandReturnCode, attributes: u32, old_s: S, new_s: S) -> bool {
    (result == RSI_SUCCESS)
    && (attributes[31..16] == 0)
    && (attributes[15..0] == NumPowerCappingDomains())
}