pub open spec fn sbi_get_impl_id_spec(value: UInt64, old_s: S, new_s: S) -> bool {
  (value == CurrentSbiImplementationId())
}