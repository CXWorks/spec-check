pub open spec fn sbi_hart_stop_spec(calling_hart: CallingHart, old_s: S, new_s: S) -> bool {
  (!HartExecutingInSMode(old_s, calling_hart) ==> !HartExecutingInSMode(new_s, calling_hart))
  && (HartOwner(old_s, calling_hart) == SBI_IMPLEMENTATION ==> HartOwner(new_s, calling_hart) == SBI_IMPLEMENTATION)
  && (CallReturns(old_s) ==> !CallReturns(new_s))
  && ((HartExecutingInSMode(old_s, calling_hart))
    ==> HartExecutingInSMode(new_s, calling_hart))
  && ((HartOwner(old_s, calling_hart) != SBI_IMPLEMENTATION)
    ==> HartOwner(new_s, calling_hart) != SBI_IMPLEMENTATION)
  && ((!CallReturns(old_s))
    ==> CallReturns(new_s))
  && (HartExecutingInSMode(old_s, calling_hart) &&
       HartOwner(old_s, calling_hart) != SBI_IMPLEMENTATION &&
       !CallReturns(old_s))
    ==> (HartExecutingInSMode(new_s, calling_hart) &&
         HartOwner(new_s, calling_hart) != SBI_IMPLEMENTATION &&
         CallReturns(new_s))
}