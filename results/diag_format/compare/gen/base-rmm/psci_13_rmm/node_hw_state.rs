pub open spec fn node_hw_state_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    (!NodeHwStateImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidNode(old_s, target_cpu, power_level) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (PowerControlView(old_s, target_cpu, power_level) == RUN ==> ResultEqual(result, HW_ON))
    && (PowerControlView(old_s, target_cpu, power_level) == POWERDOWN ==> ResultEqual(result, HW_OFF))
    && (PowerControlView(old_s, target_cpu, power_level) == STANDBY_OR_RETENTION ==> ResultEqual(result, HW_STANDBY))
}