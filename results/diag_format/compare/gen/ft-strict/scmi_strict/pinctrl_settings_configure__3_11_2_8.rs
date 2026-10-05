pub open spec fn pinctrl_settings_configure__3_11_2_8_spec(identifier: UInt32, function_id: UInt32, attributes: UInt32, configs: [UInt32; 2], status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPinOrGroup(old_s, identifier, Bits(attributes, 1, 0)) ==> ResultEqual(status, NOT_FOUND))
  && (!AreValidSettingsConfigureParameters(old_s, identifier, function_id, attributes, configs) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsSupportedConfiguration(old_s, identifier, Bits(attributes, 1, 0), function_id, attributes, configs) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetConfiguration(old_s, calling_agent, identifier, Bits(attributes, 1, 0)) ==> ResultEqual(status, DENIED))
  && (IsInUseByOtherAgent(old_s, identifier, Bits(attributes, 1, 0), calling_agent) ==> ResultEqual(status, IN_USE))
  && (Bits(attributes, 9, 2) > TransportConfigCapacity() ==> ResultEqual(status, PROTOCOL_ERROR))
  && (ResultEqual(status, SUCCESS) ==> ResultEqual(status, SUCCESS))
  && (ResultEqual(status, SUCCESS) ==> (forall i: UInt32| i < Bits(attributes, 9, 2) ==> PinOrGroupConfig(new_s, identifier, Bits(attributes, 1, 0), Bits(configs[i].type, 7, 0)) == configs[i].config_value))
  && (ResultEqual(status, SUCCESS) && Bits(attributes, 10, 10) == 1 ==> PinOrGroupFunction(new_s, identifier, Bits(attributes, 1, 0)) == function_id)
  && (ResultEqual(status, SUCCESS) && Bits(attributes, 10, 10) == 0 ==> PinOrGroupFunction(new_s, identifier, Bits(attributes, 1, 0)) == PrePinOrGroupFunction(new_s, identifier, Bits(attributes, 1, 0)))
  && ((IsValidPinOrGroup(old_s, identifier, Bits(attributes, 1, 0)) &&
       AreValidSettingsConfigureParameters(old_s, identifier, function_id, attributes, configs) &&
       IsSupportedConfiguration(old_s, identifier, Bits(attributes, 1, 0), function_id, attributes, configs) &&
       AgentMaySetConfiguration(old_s, calling_agent, identifier, Bits(attributes, 1, 0)) &&
       !(IsInUseByOtherAgent(old_s, identifier, Bits(attributes, 1, 0), calling_agent)) &&
       !(Bits(attributes, 9, 2) > TransportConfigCapacity()))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PinOrGroupConfig(new_s, identifier, Bits(attributes, 1, 0), Bits(configs[0].type, 7, 0)) == PinOrGroupConfig(old_s, identifier, Bits(attributes, 1, 0), Bits(configs[0].type, 7, 0)))
  && (result != SUCCESS
    ==> PinOrGroupConfig(new_s, identifier, Bits(attributes, 1, 0), Bits(configs[1].type, 7, 0)) == PinOrGroupConfig(old_s, identifier, Bits(attributes, 1, 0), Bits(configs[1].type, 7, 0)))
  && (result != SUCCESS
    ==> PinOrGroupFunction(new_s, identifier, Bits(attributes, 1, 0)) == PinOrGroupFunction(old_s, identifier, Bits(attributes, 1, 0)))
}