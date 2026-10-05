pub open spec fn sbi_hart_start_spec(hartid: unsigned long, start_addr: unsigned long, opaque: unsigned long, ret: struct sbiret, old_s: S, new_s: S) -> bool {
  (ret.code == 0 ==> HartStartRequestedInSupervisorMode(new_s, hartid, start_addr))
  && ((!(ret.code == 0)) ==> HartStartRequestedInSupervisorMode(new_s, hartid, start_addr))
}