pub open spec fn system_off2_spec(result: PsciReturnCode, old_s: PsciState, new_s: PsciState) -> bool {
    (result == PSCI_RETURN_INVALID_PARAMETERS)
}