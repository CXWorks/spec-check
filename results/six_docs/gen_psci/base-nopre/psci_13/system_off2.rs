pub open spec fn system_off2_spec(result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (result == PSCI_RETURN_INVALID_PARAMETERS ==> <failure conditions>)
    && (result == PSCI_RETURN_SUCCESS ==> <success conditions>)
}