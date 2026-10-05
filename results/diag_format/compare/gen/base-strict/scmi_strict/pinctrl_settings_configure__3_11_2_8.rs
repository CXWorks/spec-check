pub open spec fn pinctrl_settings_configure__3_11_2_8_spec(result: Int32, old_s: S, new_s: S) -> bool {
    (!IsValidPinOrGroup(identifier, Bits(attributes, 1, 0)) ==> ResultEqual(result, NOT_FOUND))
    && (!AreValidSettingsConfigureParameters(identifier, function_id, attributes, configs) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!IsSupportedConfiguration(identifier, Bits(attributes, 1, 0), function_id, attributes, configs) ==> ResultEqual(result, NOT_SUPPORTED))
    && (!AgentMaySetConfiguration(calling_agent, identifier, Bits(attributes, 1, 0)) ==> ResultEqual(result, DENIED))
    && (IsInUseByOtherAgent(identifier, Bits(attributes, 1, 0), calling_agent) ==> ResultEqual(result, IN_USE))
    && (Bits(attributes, 9, 2) > TransportConfigCapacity() ==> ResultEqual(result, PROTOCOL_ERROR))
    && (ResultEqual(result, SUCCESS) ==> (forall|i: UInt32| i < Bits(attributes, 9, 2) ==> PinOrGroupConfig(identifier, Bits(attributes, 1, 0), Bits(configs[i].type, 7, 0)) == configs[i].config_value))
    && (ResultEqual(result, SUCCESS) ==> (Bits(attributes, 10, 10) == 1 ==> PinOrGroupFunction(identifier, Bits(attributes, 1, 0)) == function_id))
    && (ResultEqual(result, SUCCESS) ==> (Bits(attributes, 10, 10) == 0 ==> PinOrGroupFunction(identifier, Bits(attributes, 1, 0)) == PrePinOrGroupFunction(identifier, Bits(attributes, 1, 0))))
}