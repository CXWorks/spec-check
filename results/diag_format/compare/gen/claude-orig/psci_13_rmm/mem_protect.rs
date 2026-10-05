pub open spec fn mem_protect_spec(enable: UInt64, result: Int, old_s: S, new_s: S) -> bool {
    (!IsMemProtectImplemented(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsMemProtectImplemented(old_s) ==> (
        ResultEqual(result, if MemProtectEnabled(old_s) { 1 } else { 0 })
        && ((enable != 0) == MemProtectEnabled(new_s))
    ))
}
