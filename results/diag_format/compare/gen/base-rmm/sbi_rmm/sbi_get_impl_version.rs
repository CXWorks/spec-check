pub open spec fn sbi_get_impl_version_spec(result: struct sbiret, old_s: S, new_s: S) -> bool {
    (old_s == new_s)
    && (result.value == CurrentSbiImplVersion())
}