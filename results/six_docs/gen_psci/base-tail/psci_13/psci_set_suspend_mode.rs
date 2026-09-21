pub open spec fn psci_set_suspend_mode_spec(result: int, old_s: S, new_s: S) -> bool {
    let mode: u8 = old_s.cmd_input_mode;
    let mode_valid = (mode == 0) || (mode == 1);
    let all_cores_valid = forall::<u8, S, Bits64>(|i: u8, s: S| {
        i < 64 ==> {
            let state = s.cpu_state[i as usize];
            (state == 0) || (state == 1) || (state == 2) || (state == 3)
        }
    });
    let no_cpu_suspend_used = forall::<u8, S, Bits64>(|i: u8, s: S| {
        i < 64 ==> {
            let state = s.cpu_state[i as usize];
            state != 4
        }
    });
    let switch_to_os_init_valid = (mode == 1) ==> {
        forall::<u8, S, Bits64>(|i: u8, s: S| {
            i < 64 ==> {
                let state = s.cpu_state[i as usize];
                (state == 0) || (state == 1)
            }
        })
    };
    let switch_to_platform_valid = (mode == 0) ==> {
        forall::<u8, S, Bits64>(|i: u8, s: S| {
            i < 64 ==> {
                let state = s.cpu_state[i as usize];
                (state == 0) || (state == 1)
            }
        })
    };
    let pre_fail = (!mode_valid) || (!all_cores_valid) || (!no_cpu_suspend_used);
    let pre_fail_os_init = (!switch_to_os_init_valid);
    let pre_fail_platform = (!switch_to_platform_valid);
    (!mode_valid ==> result == PSCI_INVALID_PARAMETERS)
    && (!all_cores_valid ==> result == PSCI_DENIED)
    && (!no_cpu_suspend_used ==> result == PSCI_DENIED)
    && (mode == 1 ==> (!switch_to_os_init_valid ==> result == PSCI_DENIED))
    && (mode == 0 ==> (!switch_to_platform_valid ==> result == PSCI_DENIED))
    && (mode_valid && all_cores_valid && no_cpu_suspend_used && (mode == 1 ==> switch_to_os_init_valid) && (mode == 0 ==> switch_to_platform_valid) ==> result == PSCI_SUCCESS)
    && (result == PSCI_SUCCESS ==> new_s.cpu_state == old_s.cpu_state)
}