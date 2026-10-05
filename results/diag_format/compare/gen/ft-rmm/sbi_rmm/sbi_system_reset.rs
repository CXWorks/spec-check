pub open spec fn sbi_system_reset_spec(reset_type: UInt32, reset_reason: UInt32, old_s: S, new_s: S) -> bool {
  (reset_type == 0 ==> SystemIsShutDown(new_s))
  && (reset_type == 1 ==> SystemIsColdRebooted(new_s))
  && (reset_type == 2 ==> SystemIsWarmRebooted(new_s))
  && (CallDoesNotReturn(new_s))
  && ((!(reset_type == 0) &&
       !(reset_type == 1) &&
       !(reset_type == 2))
    ==> SystemIsShutDown(new_s) &&
       SystemIsColdRebooted(new_s) &&
       SystemIsWarmRebooted(new_s))
}