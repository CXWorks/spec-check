pub open spec fn node_hw_state_spec(result: Int64, old_s: S, new_s: S) -> bool {
    (!IsNodeHwStateImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (!IsValidNode(old_s, target_cpu, power_level) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (PowerControllerViewOfNode(old_s, target_cpu, power_level) == RUN ==> ResultEqual(result, HW_ON))
    && (PowerControllerViewOfNode(old_s, target_cpu, power_level) == POWERDOWN ==> ResultEqual(result, HW_OFF))
    && (PowerControllerViewOfNode(old_s, target_cpu, power_level) == RETENTION_STANDBY ==> ResultEqual(result, HW_STANDBY))
    && (old_s == new_s)
}