pub open spec fn cpu_on_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (PsciReturnCodeIsErr(result) ==> (
        (PsciReturnCodeIs(result, PSCI_ERROR_INVALID_PARAMETERS) ==> (
            (old_s.target_cpu != 0) && (old_s.target_cpu != 1) && (old_s.target_cpu != 2) && (old_s.target_cpu != 3)
        ))
        && (PsciReturnCodeIs(result, PSCI_ERROR_INVALID_ADDRESS) ==> (
            (old_s.entry_point != 0) && (old_s.entry_point < 0x1_0000_0000)
        ))
        && (PsciReturnCodeIs(result, PSCI_ERROR_ALREADY_ON) ==> (
            (old_s.target_cpu == 0) && (old_s.target_cpu == 1) && (old_s.target_cpu == 2) && (old_s.target_cpu == 3)
        ))
        && (PsciReturnCodeIs(result, PSCI_ERROR_ON_PENDING) ==> (
            (old_s.target_cpu == 0) && (old_s.target_cpu == 1) && (old_s.target_cpu == 2) && (old_s.target_cpu == 3)
        ))
        && (PsciReturnCodeIs(result, PSCI_ERROR_INTERNAL_FAILURE) ==> (
            (old_s.target_cpu == 0) && (old_s.target_cpu == 1) && (old_s.target_cpu == 2) && (old_s.target_cpu == 3)
        ))
        && (PsciReturnCodeIs(result, PSCI_ERROR_DENIED) ==> (
            (old_s.target_cpu == 0) && (old_s.target_cpu == 1) && (old_s.target_cpu == 2) && (old_s.target_cpu == 3)
        ))
    ))
    && (PsciReturnCodeIsOk(result) ==> (
        (old_s.target_cpu == 0) && (old_s.target_cpu == 1) && (old_s.target_cpu == 2) && (old_s.target_cpu == 3)
    ))
}