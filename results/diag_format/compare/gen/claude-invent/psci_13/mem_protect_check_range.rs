pub open spec fn mem_protect_check_range_spec(base: UInt64, length: UInt64, result: PsciReturnCode, old_s: S, new_s: S) -> bool {
    (!MemProtectRangeIsProtected(old_s, base, length) ==> (result == DENIED && new_s == old_s))
    && (MemProtectRangeIsProtected(old_s, base, length) ==> (result == SUCCESS && new_s == old_s))
}
