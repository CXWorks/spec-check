pub open spec fn node_hw_state_spec(target_cpu: MPIDR-derived, power_level: IMPLEMENTATION DEFINED, result: Result<(), (HW_ON(0), HW_OFF(1) or HW_STANDBY(2))>, old_s: S, new_s: S) -> bool {
  (!NodeHwStateImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
  && (!IsValidNode(old_s, target_cpu, power_level) ==> ResultEqual(result, INVALID_PARAMETERS))
  && (result.is_Ok() && (PowerControlView(old_s, target_cpu, power_level) == RUN) ==> ResultEqual(result, HW_ON))
  && (result.is_Ok() && (PowerControlView(old_s, target_cpu, power_level) == POWERDOWN) ==> ResultEqual(result, HW_OFF))
  && (result.is_Ok() && (PowerControlView(old_s, target_cpu, power_level) == STANDBY_OR_RETENTION) ==> ResultEqual(result, HW_STANDBY))
  && ((!(NodeHwStateImplemented(old_s)) &&
       IsValidNode(old_s, target_cpu, power_level))
    ==> result.is_Ok())
}