pub open spec fn sbi_probe_extension_spec(extension_id: i64, value: i64, old_s: S, new_s: S) -> bool {
    (!SbiExtensionAvailable(old_s, extension_id) ==> value == 0)
    && (SbiExtensionAvailable(old_s, extension_id) ==> value != 0)
    && (new_s == old_s)
}
