pub open spec fn psci_features_spec(
    result: PsciReturnCode,
    old_s: S,
    new_s: S,
    psci_func_id: u32,
) -> bool {
    (PsciIsFunctionSupported(old_s, psci_func_id) ==> result == PsciReturnCode::PSCI_SUCCESS) && (
    !PsciIsFunctionSupported(old_s, psci_func_id) ==> result == PsciReturnCode::PSCI_NOT_SUPPORTED)
        && (new_s == old_s)
}