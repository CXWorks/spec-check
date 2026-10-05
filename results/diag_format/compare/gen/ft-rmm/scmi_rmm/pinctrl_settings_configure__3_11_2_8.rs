pub open spec fn pinctrl_settings_configure__3_11_2_8_spec(identifier: UInt32, function_id: UInt32, attributes: UInt32, num_configs: UInt8, selector: UInt8, configs: [UInt32; 2], config_values: [UInt32; 2], status: Int32, old_s: S, new_s: S) -> bool {
  (!IsValidPinOrGroup(old_s, identifier, selector) ==> ResultEqual(status, NOT_FOUND))
  && (attributes[31:11] != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (selector > 1 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (num_configs == 0 && function_id_valid == 0 ==> ResultEqual(status, INVALID_PARAMETERS))
  && (Exists(i < num_configs, configs[i].type[31:8] != 0) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (!IsSortedIncreasingByConfigType(old_s, configs, num_configs) ==> ResultEqual(status, INVALID_PARAMETERS))
  && (num_configs > TransportMaxConfigs() ==> ResultEqual(status, PROTOCOL_ERROR))
  && (!IsConfigSupported(old_s, identifier, selector, configs, num_configs, function_id_valid, function_id) ==> ResultEqual(status, NOT_SUPPORTED))
  && (!AgentMaySetConfig(old_s, calling_agent, identifier, selector) ==> ResultEqual(status, DENIED))
  && (IsInUseByOtherAgent(old_s, identifier, selector, calling_agent) ==> ResultEqual(status, IN_USE))
  && (ResultEqual(status, SUCCESS) ==> ForAll(i < num_configs, PinOrGroupConfig(new_s, identifier, selector, configs[i].config_type) == configs[i].config_value))
  && (ResultEqual(status, SUCCESS) && function_id_valid == 1 && function_id != 0xFFFFFFFF ==> SelectedFunction(new_s, identifier, selector) == function_id)
  && (ResultEqual(status, SUCCESS) && function_id_valid == 1 && function_id == 0xFFFFFFFF ==> !HasFunctionEnabled(new_s, identifier, selector))
  && (ResultEqual(status, SUCCESS) && function_id_valid == 0 ==> SelectedFunction(new_s, identifier, selector) == SelectedFunction(old_s, identifier, selector))
  && ((!(IsValidPinOrGroup(old_s, identifier, selector)) &&
       !(attributes[31:11] != 0) &&
       !(selector > 1) &&
       !(num_configs == 0 && function_id_valid == 0) &&
       !(Exists(i < num_configs, configs[i].type[31:8] != 0)) &&
       IsSortedIncreasingByConfigType(old_s, configs, num_configs) &&
       !(num_configs > TransportMaxConfigs()) &&
       IsConfigSupported(old_s, identifier, selector, configs, num_configs, function_id_valid, function_id) &&
       AgentMaySetConfig(old_s, calling_agent, identifier, selector) &&
       !(IsInUseByOtherAgent(old_s, identifier, selector, calling_agent)))
    ==> ResultEqual(status, SUCCESS))
  && (result != SUCCESS
    ==> PinOrGroupConfig(new_s, identifier, selector, configs[0].config_type) == PinOrGroupConfig(old_s, identifier, selector, configs[0].config_type))
  && (result != SUCCESS
    ==> PinOrGroupConfig(new_s, identifier, selector, configs[1].config_type) == PinOrGroupConfig(old_s, identifier, selector, configs[1].config_type))
  && (result != SUCCESS
    ==> SelectedFunction(new_s, identifier, selector) == SelectedFunction(old_s, identifier, selector))
  && (result != SUCCESS
    ==> SelectedFunction(new_s, identifier, selector) == SelectedFunction(old_s, identifier, selector))
}