pub open spec fn sbi_pmu_counter_config_matching_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    let counter_idx_base = old_s.cmd_input_counter_idx_base;
    let counter_idx_mask = old_s.cmd_input_counter_idx_mask;
    let config_flags = old_s.cmd_input_config_flags;
    let event_idx = old_s.cmd_input_event_idx;
    let event_data = old_s.cmd_input_event_data;

    // Failure condition: Reserved bits (8:(XLEN-1)) are not zero
    (config_flags & ((1u64 << 8) - 1) != 0 ==> result.is_Err())
    // Failure condition: If SKIP_MATCH is not set, no matching counter was found
    (!((config_flags & 1u64) != 0) ==> result.is_Err())
    // Success condition: SKIP_MATCH is set OR a matching counter was found and configured
    (((config_flags & 1u64) != 0) || (result.is_Ok()))
    && (result.is_Ok() ==> {
        // If SKIP_MATCH is set, the first counter in the range is selected
        ((config_flags & 1u64) != 0 ==> {
            let first_counter = counter_idx_base;
            // The selected counter must be within the specified range
            (first_counter as int) >= (counter_idx_base as int)
            && (first_counter as int) < ((counter_idx_base as int) + (counter_idx_mask as int) + 1)
        })
        // If SKIP_MATCH is not set, a matching counter was found
        (!((config_flags & 1u64) != 0) ==> {
            // The counter must be in the specified range
            let selected_counter = old_s.cmd_output_selected_counter;
            (selected_counter as int) >= (counter_idx_base as int)
            && (selected_counter as int) < ((counter_idx_base as int) + (counter_idx_mask as int) + 1)
            // The counter must not have been started (or enabled) before
            (!old_s.counter_started(selected_counter))
            // The counter must be able to monitor the specified event
            (old_s.counter_can_monitor(selected_counter, event_idx))
            // The counter value is cleared if CLEAR_VALUE flag is set
            ((config_flags & 2u64) != 0 ==> new_s.counter_value(selected_counter) == 0)
            // The counter is started if AUTO_START flag is set
            ((config_flags & 4u64) != 0 ==> new_s.counter_started(selected_counter))
            // Event filtering hints are applied if SET_VUINH, SET_VSINH, SET_UINH, SET_SINH, SET_MINH flags are set
            ((config_flags & 24u64) != 0 ==> {
                let inh_flags = config_flags & 24u64;
                if (inh_flags & 8u64) != 0 { new_s.counter_vu_inhibited(selected_counter) }
                if (inh_flags & 16u64) != 0 { new_s.counter_vs_inhibited(selected_counter) }
                if (inh_flags & 32u64) != 0 { new_s.counter_u_inhibited(selected_counter) }
                if (inh_flags & 64u64) != 0 { new_s.counter_s_inhibited(selected_counter) }
                if (inh_flags & 128u64) != 0 { new_s.counter_m_inhibited(selected_counter) }
            })
        })
    })
}