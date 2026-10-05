pub open spec fn system_power_state_notifier__3_4_3_1_spec(agent_id: UInt32, flags: UInt32, system_state: UInt32, timeout: UInt32, result: Result<(), PsciStatusCode>, old_s: S, new_s: S) -> bool {
  (AgentRegisteredForSystemPowerStateNotify(new_s, recipient) &&
   Bits(new_s, flags, 31, 1) == 0 &&
   system_state <= 0x4 || system_state >= 0x80000000 &&
   (system_state == 0x0 && Bits(new_s, flags, 0, 0) == 1 && !PlatformImposesShutdownTimeout(new_s)) ==> timeout == 0 &&
   (system_state == 0x0 && Bits(new_s, flags, 0, 0) == 1 && PlatformImposesShutdownTimeout(new_s) && ShutdownTimeoutExpired(new_s, timeout) && !SystemShutdownRequestReceived(new_s)) ==> PlatformMayForceSystemShutdown(new_s))
}