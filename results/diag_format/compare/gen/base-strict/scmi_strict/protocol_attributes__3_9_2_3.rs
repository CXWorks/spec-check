pub open spec fn protocol_attributes__3_9_2_3_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    ResultEqual(result, SUCCESS)
    && Bits(attributes, 31, 16) == 0
    && Bits(attributes, 15, 0) == NumVoltageDomains()
    && old_s == new_s
}