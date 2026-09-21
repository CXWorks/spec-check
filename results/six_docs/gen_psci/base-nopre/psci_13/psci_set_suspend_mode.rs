pub open spec fn psci_set_suspend_mode_spec(result: RsiCommandReturnCode, old_s: S, new_s: S) -> bool {
    let mode = old_s.cmd_input_mode as int;
    let is_valid_mode = mode == 0 || mode == 1;
    let is_platform_coordinated = old_s.psci_coordination_mode == PSCI_COORDINATION_PLATFORM;
    let is_os_initiated = old_s.psci_coordination_mode == PSCI_COORDINATION_OS;
    let all_cores_off_or_not_booted = forall(|c: u64| c < old_s.num_cores) => {
        let state = old_s.cpu_state(c);
        state == CPU_STATE_OFF || state == CPU_STATE_NOT_BOOTED
    };
    let no_core_used_suspend = forall(|c: u64| c < old_s.num_cores) => {
        old_s.cpu_suspend_used(c) == false
    };
    let all_other_cores_off = forall(|c: u64| c < old_s.num_cores && c != old_s.calling_core) => {
        old_s.cpu_state(c) == CPU_STATE_OFF
    };
    let switch_to_os_initiated = is_platform_coordinated && mode == 1;
    let switch_to_platform_coordinated = is_os_initiated && mode == 0;
    let switch_to_os_initiated_valid = switch_to_os_initiated && all_cores_off_or_not_booted && no_core_used_suspend;
    let switch_to_platform_coordinated_valid = switch_to_platform_coordinated;
    let success = is_valid_mode && (switch_to_os_initiated_valid || switch_to_platform_coordinated_valid);
    (mode != 0 && mode != 1 ==> result == RSI_ERROR_INPUT)
    && (switch_to_os_initiated && !switch_to_os_initiated_valid ==> result == RSI_ERROR_STATE)
    && (switch_to_platform_coordinated && !switch_to_platform_coordinated_valid ==> result == RSI_ERROR_STATE)
    && (success ==> result == RSI_SUCCESS)
    && (success ==> new_s.psci_coordination_mode == mode)
    && (!success ==> new_s.psci_coordination_mode == old_s.psci_coordination_mode)
    && (success ==> forall(|c: u64| c < old_s.num_cores) => {
        new_s.cpu_state(c) == old_s.cpu_state(c)
    })
    && (success ==> forall(|c: u64| c < old_s.num_cores) => {
        new_s.cpu_suspend_used(c) == old_s.cpu_suspend_used(c)
    })
}