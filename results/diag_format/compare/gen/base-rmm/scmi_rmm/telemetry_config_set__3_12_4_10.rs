pub open spec fn telemetry_config_set__3_12_4_10_spec(status: Int32, old_s: S, new_s: S) -> bool {
    (!AreValidTelemetryConfigParams(old_s, new_s.group_identifier, new_s.control, new_s.sampling_rate) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (new_s.control.reserved != 0 ==> ResultEqual(status, INVALID_PARAMETERS))
    && (new_s.control.selector > 2 ==> ResultEqual(status, INVALID_PARAMETERS))
    && (ProtocolAttributes(old_s).attributes_1[18] == 0 && new_s.control.selector != 2 ==> ResultEqual(status, INVALID_PARAMETERS))
    && (new_s.control.enable == 1 && new_s.control.mode > 2 ==> ResultEqual(status, INVALID_PARAMETERS))
    && (NoDeEnabled(old_s, new_s.control.selector, new_s.group_identifier, new_s.control) ==> ResultEqual(status, INVALID_PARAMETERS))
    && (new_s.control.enable == 1 && EnabledDeOrGroupLimitReached(old_s, new_s.control.selector, new_s.group_identifier, new_s.control.mode) ==> ResultEqual(status, OUT_OF_RANGE))
    && (ResultEqual(status, SUCCESS) ==> (forall de in TargetDes(old_s, new_s.control.selector, new_s.group_identifier): TelemetryEnabled(de) == new_s.control.enable))
    && (new_s.control.enable == 1 ==> (forall de in TargetDes(old_s, new_s.control.selector, new_s.group_identifier): TelemetryMode(de) == new_s.control.mode || (new_s.control.mode == 0 && !InterfaceSupportsOnDemand(de) && TelemetryMode(de) == 1)))
    && ((new_s.control.enable == 1 && new_s.control.mode != 2) ==> (forall de in TargetDes(old_s, new_s.control.selector, new_s.group_identifier): SamplingRate(de) == new_s.sampling_rate.sec * 10 as int ^ new_s.sampling_rate.exponent as int))
}