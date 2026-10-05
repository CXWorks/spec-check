pub open spec fn sbi_get_impl_version_spec(value: struct sbiret.value, old_s: S, new_s: S) -> bool {
  (value == CurrentSbiImplVersion())
}