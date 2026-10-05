pub open spec fn system_power_state_notifier__3_4_3_1_spec(result: Result<(), RmiStatusCode>, old_s: S, new_s: S) -> bool {
    (result.is_Ok() ==> AgentRegisteredForSystemPowerStateNotify(recipient))
    && (result.is_Ok() ==> Bits64(new_s, 31, 1) == 0)
    && (result.is_Ok() ==> new_s.system_state <= 0x4 || new_s.system_state >= 0x80000000)
    && (result.is_Ok() ==> (new_s.system_state == 0x0 && Bits64(new_s, 0, 0) == 1 && !PlatformImposesShutdownTimeout()) ==> new_s.timeout == 0)
    && (result.is_Ok() ==> (new_s.system_state == 0x0 && Bits64(new_s, 0, 0) == 1 && PlatformImposesShutdownTimeout() && ShutdownTimeoutExpired(new_s.timeout) && !SystemShutdownRequestReceived()) ==> PlatformMayForceSystemShutdown())
    && (result.is_Ok() ==> old_s == new_s)
}