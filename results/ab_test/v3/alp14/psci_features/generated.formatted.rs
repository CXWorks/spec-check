pub open spec fn psci_features_spec(
    psci_func_id: u32,
    result: PsciReturnCode,
    old_s: S,
    new_s: S,
) -> bool {
    (IsPsciSupported(psci_func_id) ==> result == PSCI_SUCCESS) && (!IsPsciSupported(psci_func_id)
        ==> result == PSCI_NOT_SUPPORTED) && (old_s == new_s)
}