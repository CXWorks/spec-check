pub open spec fn sbi_get_impl_id_spec(impl_id: UInt64, old_s: S, new_s: S) -> bool {
    (impl_id == SbiImplementationId(old_s))
    && (new_s == old_s)
}
