pub open spec fn system_power_state_notifier__3_4_3_1_spec(agent_id: u32, flags: u32, system_state: u32, timeout: u32, old_s: S, new_s: S) -> bool {
    ((flags >> 1u32) == 0u32)
    && !((system_state as int) >= 0x5 && (system_state as int) <= 0x7FFF_FFFF)
    && (!PlatformImposesShutdownTimeout(old_s) ==> timeout == 0u32)
}
