pub open spec fn node_hw_state_spec(target_cpu: Bits64, power_level: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!NodeHwStateImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidNode(target_cpu, power_level) ==> ResultEqual(result, INVALID_PARAMETERS))
    && ((NodeHwStateImplemented() && IsValidNode(target_cpu, power_level)) ==> (
        ((PowerControlView(target_cpu, power_level) == RUN) ==> ResultEqual(result, HW_ON))
        && ((PowerControlView(target_cpu, power_level) == POWERDOWN) ==> ResultEqual(result, HW_OFF))
        && ((PowerControlView(target_cpu, power_level) == STANDBY_OR_RETENTION) ==> ResultEqual(result, HW_STANDBY))
    ))
}
