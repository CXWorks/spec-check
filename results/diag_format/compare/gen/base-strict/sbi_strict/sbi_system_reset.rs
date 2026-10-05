pub open spec fn sbi_system_reset_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (!CallReturns() ==> result == sbiret::SBI_SUCCESS)
    && (result == sbiret::SBI_SUCCESS ==> !CallReturns())
    && (result == sbiret::SBI_SUCCESS ==> (old_s.reset_type == 0x00000000 ==> SystemShutdown()))
    && (result == sbiret::SBI_SUCCESS ==> (old_s.reset_type == 0x00000001 ==> SystemColdReboot()))
    && (result == sbiret::SBI_SUCCESS ==> (old_s.reset_type == 0x00000002 ==> SystemWarmReboot()))
    && (result == sbiret::SBI_SUCCESS ==> (old_s.reset_type == 0x00000000 && SupervisorRunsNatively(old_s) ==> PhysicalPowerDownOfEntireSystem()))
    && (result == sbiret::SBI_SUCCESS ==> (old_s.reset_type == 0x00000001 && SupervisorRunsNatively(old_s) ==> PhysicalPowerCycleOfEntireSystem()))
    && (result == sbiret::SBI_SUCCESS ==> (old_s.reset_type == 0x00000002 && SupervisorRunsNatively(old_s) ==> PowerCycleOfMainProcessorAndPartsOfSystem()))
}