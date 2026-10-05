pub open spec fn node_hw_state_spec(target_cpu: UInt64, power_level: UInt64, result: Int64, old_s: S, new_s: S) -> bool {
  (!IsNodeHwStateImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidNode(old_s, target_cpu, power_level) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result == HW_ON ==> PowerControllerViewOfNode(new_s, target_cpu, power_level) == RUN ==> ResultEqual(result, HW_ON))
  && (result == HW_OFF ==> PowerControllerViewOfNode(new_s, target_cpu, power_level) == POWERDOWN ==> ResultEqual(result, HW_OFF))
  && (result == HW_STANDBY ==> PowerControllerViewOfNode(new_s, target_cpu, power_level) == RETENTION_STANDBY ==> ResultEqual(result, HW_STANDBY))
  && ((IsNodeHwStateImplemented(old_s) &&
       IsValidNode(old_s, target_cpu, power_level))
    ==> result == HW_ON || result == HW_OFF || result == HW_STANDBY || result == NOT_SUPPORTED || result == INVALID_PARAMETERS)
}