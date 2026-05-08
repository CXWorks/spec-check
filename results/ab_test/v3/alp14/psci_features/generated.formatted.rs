pub open spec fn PSCI_FEATURES_spec(
    old_s: S,
    new_s: S,
    psci_func_id: u32,
    result: PsciReturnCode,
) -> bool {
    old_s == new_s && ((PsciIsSupportedFunction(psci_func_id) ==> result
        == PsciReturnCode::PSCI_SUCCESS) && (!PsciIsSupportedFunction(psci_func_id) ==> result
        == PsciReturnCode::PSCI_NOT_SUPPORTED))
}