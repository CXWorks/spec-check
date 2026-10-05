pub open spec fn node_hw_state_spec(target_cpu: Bits64, power_level: Bits32, result: Bits64, old_s: S, new_s: S) -> bool {
    (!PsciNodeHwStateImplemented(old_s) ==> (result == NOT_SUPPORTED && new_s == old_s))
    && ((PsciNodeHwStateImplemented(old_s) && !IsValidPowerNode(old_s, target_cpu, power_level)) ==> (result == INVALID_PARAMETERS && new_s == old_s))
    && ((PsciNodeHwStateImplemented(old_s) && IsValidPowerNode(old_s, target_cpu, power_level)) ==> (
        (result == HW_ON || result == HW_OFF || result == HW_STANDBY)
        && (NodeHwInRunState(old_s, target_cpu, power_level) ==> result == HW_ON)
        && (NodeHwInPowerdownState(old_s, target_cpu, power_level) ==> result == HW_OFF)
        && (NodeHwInStandbyState(old_s, target_cpu, power_level) ==> result == HW_STANDBY)
        && new_s == old_s
    ))
}
