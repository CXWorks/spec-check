pub open spec fn sbi_hart_start_spec(result: u64, old_s: S, new_s: S) -> bool {
    (HartStartRequested(new_s, hartid, start_addr, opaque) ==> true)
}