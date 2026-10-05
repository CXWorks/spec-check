pub open spec fn telemetry_config_set__3_12_4_10_spec(group_identifier: UInt32, control: Control, sampling_rate: SamplingRate, status: Int32, old_s: S, new_s: S) -> bool {
  (!AreValidTelemetryConfigParams(old_s, group_identifier, control, sampling_rate) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (control.reserved != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (control.selector > 2 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (ProtocolAttributes(old_s).attributes_1[18] == 0 && control.selector != 2 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (control.enable == 1 && control.mode > 2 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (NoDeEnabled(old_s, control.selector, group_identifier, control) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (control.enable == 1 && EnabledDeOrGroupLimitReached(old_s, control.selector, group_identifier, control.mode) ==> ResultEqual(result, OUT_OF_RANGE))
  && (result == RSI_SUCCESS ==> ResultEqual(result, SUCCESS))
  && (result == RSI_SUCCESS ==> forall de in TargetDes(old_s, control.selector, group_identifier): TelemetryEnabled(new_s, de) == control.enable)
  && (result == RSI_SUCCESS && control.enable == 1 ==> forall de in TargetDes(old_s, control.selector, group_identifier): TelemetryMode(new_s, de) == control.mode || (control.mode == 0 && !InterfaceSupportsOnDemand(new_s, de) && TelemetryMode(new_s, de) == 1))
  && (result == RSI_SUCCESS && (control.enable == 1 && control.mode != 2) ==> forall de in TargetDes(old_s, control.selector, group_identifier): SamplingRate(new_s, de) == sampling_rate.sec * pow(10, sampling_rate.exponent as nat))
  && ((AreValidTelemetryConfigParams(old_s, group_identifier, control, sampling_rate) &&
       !(control.reserved != 0) &&
       !(control.selector > 2) &&
       !(ProtocolAttributes(old_s).attributes_1[18] == 0 && control.selector != 2) &&
       !(control.enable == 1 && control.mode > 2) &&
       !(NoDeEnabled(old_s, control.selector, group_identifier, control)) &&
       !(control.enable == 1 && EnabledDeOrGroupLimitReached(old_s, control.selector, group_identifier, control.mode)))
    ==> result == RSI_SUCCESS)
  && (result != RSI_SUCCESS
    ==> TelemetryEnabled(new_s, d) == TelemetryEnabled(old_s, d))
  && (result != RSI_SUCCESS
    ==> TelemetryMode(new_s, d) == TelemetryMode(old_s, d))
  && (result != RSI_SUCCESS
    ==> SamplingRate(new_s, d) == SamplingRate(old_s, d))
}