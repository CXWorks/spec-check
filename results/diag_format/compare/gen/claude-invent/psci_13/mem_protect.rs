pub open spec fn mem_protect_spec(enable: UInt32, result: i32, old_s: S, new_s: S) -> bool {
    (!MemProtectImplemented(old_s) ==> (result == NOT_SUPPORTED
        && MemProtectEnabled(new_s) == MemProtectEnabled(old_s)))
    && (MemProtectImplemented(old_s) ==> (
        (!MemProtectEnabled(old_s) ==> result == 0)
        && (MemProtectEnabled(old_s) ==> result == 1)
        && ((enable != 0) ==> MemProtectEnabled(new_s))
        && ((enable == 0) ==> !MemProtectEnabled(new_s))
    ))
}
