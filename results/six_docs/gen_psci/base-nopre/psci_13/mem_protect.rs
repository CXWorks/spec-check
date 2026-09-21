pub open spec fn mem_protect_spec(result: u64, old_s: S, new_s: S) -> bool {
    (result == 0 ==> old_s.mem_protect_enabled == false)
    && (result == 1 ==> old_s.mem_protect_enabled == true)
    && (result == NOT_SUPPORTED ==> !PsciFeaturesPresent(old_s, MEM_PROTECT))
}