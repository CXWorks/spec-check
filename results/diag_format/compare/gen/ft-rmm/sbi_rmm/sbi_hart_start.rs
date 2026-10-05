pub open spec fn sbi_hart_start_spec(hartid: UInt, start_addr: Address, opaque: UInt, old_s: S, new_s: S) -> bool {
  (HartStartRequested(new_s, hartid, start_addr, opaque))
}