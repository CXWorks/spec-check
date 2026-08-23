pub open spec fn psci_set_suspend_mode_spec(mode: UInt32, result: int32, old_s: S, new_s: S) -> bool {
  (result != 0 ==> (mode == 0 || mode == 1))
  && ((!( (CpuIsOn(old_s, 0) || CpuIsOn(old_s, 1)) &&
        !(CpuIsOn(old_s, 0) || CpuIsOn(old_s, 1)) &&
        !(CpuIsOn(old_s, 0) || CpuIsOn(old_s, 1)) &&
        !(CpuIsOn(old_s, 0) || CpuIsOn(old_s, 1)))
    ==> result == DENIED)
  )
  && ((result == SUCCESS
       ==> true)
  )
}