pub open spec fn mem_protect_spec(result: Int, old_s: S, new_s: S) -> bool {
    (!IsMemProtectImplemented() ==> ResultEqual(result, NOT_SUPPORTED))
    && (ResultEqual(result, Old(MemProtectEnabled()) ? 1 : 0))
    && ((old_s.MemProtectEnabled() != 0) == (old_s.MemProtectEnabled() == new_s.MemProtectEnabled()))
}