pub open spec fn protocol_attributes_spec(result: Int32, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (true ==> result == 0)
    && (true ==> (attributes & 0xFFFF_0000) == 0)
    && (true ==> (attributes & 0xFFFF) == NumResetDomains())
}