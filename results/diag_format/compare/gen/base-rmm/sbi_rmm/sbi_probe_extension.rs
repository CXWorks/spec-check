pub open spec fn sbi_probe_extension_spec(result: i64, old_s: S, new_s: S) -> bool {
    (!IsExtensionAvailable(extension_id) ==> result == 0)
    && (IsExtensionAvailable(extension_id) ==> (result == 1 || (result != 0 && IsImplementationDefinedProbeValue(result))))
    && (old_s == new_s)
}