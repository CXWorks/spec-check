pub open spec fn sbi_system_reset_spec(reset_type: UInt32, reset_reason: UInt32, old_s: S, new_s: S) -> bool {
  (reset_type == 0x00000000 ==> SystemShutdown(new_s))
  && (reset_type == 0x00000001 ==> SystemColdReboot(new_s))
  && (reset_type == 0x00000002 ==> SystemWarmReboot(new_s))
  && (reset_type == 0x00000000 && SupervisorRunsNatively(old_s) ==> PhysicalPowerDownOfEntireSystem(new_s))
  && (reset_type == 0x00000001 && SupervisorRunsNatively(old_s) ==> PhysicalPowerCycleOfEntireSystem(new_s))
  && (reset_type == 0x00000002 && SupervisorRunsNatively(old_s) ==> PowerCycleOfMainProcessorAndPartsOfSystem(new_s))
}