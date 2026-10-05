pub open spec fn protocol_attributes__3_2_2_3_spec(status: i32, attributes: u32, old_s: S, new_s: S) -> bool {
    (status == 0 ==> (
        (attributes >> 16u32) == 0u32
        && (((attributes >> 8u32) & 0xFFu32) as int) == (if PlatformSupportsAgentDiscovery(old_s) { NumAgentsInSystem(old_s) } else { 0int })
        && ((attributes & 0xFFu32) as int) == NumImplementedProtocolsExcludingBase(old_s)
    ))
    && new_s == old_s
}
