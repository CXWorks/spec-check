pub open spec fn sbi_probe_extension_spec(value: Int64, extension_id: Int64, old_s: S, new_s: S) -> bool {
    (!IsExtensionAvailable(extension_id) ==> value == 0)
    && (IsExtensionAvailable(extension_id) ==> value != 0)
    && (IsExtensionAvailable(extension_id) && !ImplDefinesProbeValue(extension_id) ==> value == 1)
    && (IsExtensionAvailable(extension_id) && ImplDefinesProbeValue(extension_id) ==> value == ImplProbeValue(extension_id))
    && (old_s == new_s)
}