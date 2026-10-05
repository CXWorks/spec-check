pub open spec fn protocol_attributes__3_2_2_3_spec(attributes: uint32, old_s: S, new_s: S) -> bool {
  (attributes[31:16] == 0)
  && (attributes[15:8] == NumAgentsInSystem())
  && (attributes[7:0] == NumImplementedProtocolsExcludingBase())
  && (!PlatformSupportsAgentDiscovery(old_s) ==> attributes[15:8] == 0)
  && ((!(attributes[31:16] == 0) ||
       !(attributes[15:8] == NumAgentsInSystem()) ||
       !(attributes[7:0] == NumImplementedProtocolsExcludingBase()) ||
       PlatformSupportsAgentDiscovery(old_s))
    ==> (attributes[15:8] == attributes[15:8]))
}