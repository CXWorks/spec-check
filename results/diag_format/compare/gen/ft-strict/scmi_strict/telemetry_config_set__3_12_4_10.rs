pub open spec fn telemetry_config_set__3_12_4_10_spec(group_identifier: UInt32, control: UInt32, sampling_rate: UInt32, result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
  (Bits(control, 31, 9) != 0 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(control, 8, 5) > 2 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(ProtocolAttributes1(), 18, 18) == 0 && Bits(control, 8, 5) != 2 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(control, 8, 5) == 1 && !IsValidEventGroup(old_s, group_identifier) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(control, 0, 0) == 1 && Bits(control, 4, 1) > 2 ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(control, 0, 0) == 1 && !AnyDeEnabled(old_s, Bits(control, 8, 5), group_identifier) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (Bits(control, 0, 0) == 1 && EnabledDeOrGroupLimitReached(old_s, Bits(control, 8, 5), group_identifier, Bits(control, 4, 1)) ==> ResultEqual(result, OUT_OF_RANGE))
  && (result.is_Ok() ==> ResultEqual(result, SUCCESS))
  && (result.is_Ok() && Bits(control, 0, 0) == 1 ==> TelemetryEnabled(new_s, Bits(control, 8, 5), group_identifier))
  && (result.is_Ok() && Bits(control, 0, 0) == 0 ==> !TelemetryEnabled(new_s, Bits(control, 8, 5), group_identifier))
  && (result.is_Ok() && Bits(control, 0, 0) == 1 && !(Bits(control, 4, 1) == 0 && !AllInterfacesSupportOnDemand(old_s)) ==> TelemetryMode(new_s, Bits(control, 8, 5), group_identifier) == Bits(control, 4, 1))
  && (result.is_Ok() && Bits(control, 0, 0) == 1 && Bits(control, 4, 1) == 0 && !AllInterfacesSupportOnDemand(old_s) ==> TelemetryMode(new_s, Bits(control, 8, 5), group_identifier) == 1)
  && (result.is_Ok() && Bits(control, 0, 0) == 1 && Bits(control, 4, 1) == 2 ==> TelemetryDisabledAfterReadingComplete(new_s, Bits(control, 8, 5), group_identifier))
  && (result.is_Ok() && Bits(control, 0, 0) == 1 && Bits(control, 4, 1) != 2 ==> SamplingRate(new_s, Bits(control, 8, 5), group_identifier) == Bits(sampling_rate, 20, 5) * Pow10(new_s, SignedBits(sampling_rate, 4, 0) as int))
  && ((!(Bits(control, 31, 9) != 0) &&
       !(Bits(control, 8, 5) > 2) &&
       !(Bits(ProtocolAttributes1(), 18, 18) == 0 && Bits(control, 8, 5) != 2) &&
       !(Bits(control, 8, 5) == 1 && !IsValidEventGroup(old_s, group_identifier)) &&
       !(Bits(control, 0, 0) == 1 && Bits(control, 4, 1) > 2) &&
       !(Bits(control, 0, 0) == 1 && !AnyDeEnabled(old_s, Bits(control, 8, 5), group_identifier)) &&
       !(Bits(control, 0, 0) == 1 && EnabledDeOrGroupLimitReached(old_s, Bits(control, 8, 5), group_identifier, Bits(control, 4, 1))))
    ==> result.is_Ok())
  && (result.is_Err()
    ==> TelemetryEnabled(new_s, Bits(control, 8, 5), group_identifier) == TelemetryEnabled(old_s, Bits(control, 8, 5), group_identifier))
  && (result.is_Err()
    ==> TelemetryMode(new_s, Bits(control, 8, 5), group_identifier) == TelemetryMode(old_s, Bits(control, 8, 5), group_identifier))
  && (result.is_Err()
    ==> SamplingRate(new_s, Bits(control, 8, 5), group_identifier) == SamplingRate(old_s, Bits(control, 8, 5), group_identifier))
}