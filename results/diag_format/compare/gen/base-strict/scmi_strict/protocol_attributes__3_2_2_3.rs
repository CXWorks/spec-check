pub open spec fn protocol_attributes__3_2_2_3_spec(result: RsiCommandReturnCode, attributes: UInt32, old_s: S, new_s: S) -> bool {
    (true ==> result == RSI_SUCCESS)
    && (true ==> (attributes & 0xFFFF0000) == 0)
    && (PlatformSupportsAgentDiscovery() ==> (attributes & 0xFF00) == (NumAgentsInSystem() << 8))
    && (!PlatformSupportsAgentDiscovery() ==> (attributes & 0xFF00) == 0)
    && ((attributes & 0xFF) == NumImplementedProtocolsExcludingBase())
    && (true ==> old_s == new_s)
}