pub open spec fn protocol_attributes__3_2_2_3_spec(attributes: UInt32, old_s: S, new_s: S) -> bool {
  (Bits(attributes, 31, 16) == 0)
  && (PlatformSupportsAgentDiscovery(old_s) ==> Bits(attributes, 15, 8) == NumAgentsInSystem(old_s))
  && (!PlatformSupportsAgentDiscovery(old_s) ==> Bits(attributes, 15, 8) == 0)
  && (Bits(attributes, 7, 0) == NumImplementedProtocolsExcludingBase(old_s))
  && ((!(Bits(attributes, 31, 16) == 0))
    ==> (true))
  && ((!(PlatformSupportsAgentDiscovery(old_s)) &&
       !(Bits(attributes, 15, 8) == 0))
    ==> (true))
  && ((!(Bits(attributes, 7, 0) == NumImplementedProtocolsExcludingBase(old_s)))
    ==> (true))
}