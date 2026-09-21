pub open spec fn sbi_probe_extension_spec(result: int, old_s: S, new_s: S) -> bool {
    (result == 0 ==> !SbiExtensionAvailable(old_s, extension_id))
    && (result == 1 ==> SbiExtensionAvailable(old_s, extension_id))
    && (result != 0 && result != 1 ==> true)
    && (old_s == new_s)
}