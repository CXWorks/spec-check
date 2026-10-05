pub open spec fn sbi_hart_stop_spec(result: sbiret, old_s: S, new_s: S) -> bool {
    (!HartExecutingInSMode(old_s, calling_hart) ==> result.ret == 0)
    && (HartExecutingInSMode(old_s, calling_hart) ==> result.ret == 0)
    && (HartOwner(old_s, calling_hart) == SBI_IMPLEMENTATION ==> result.ret == 0)
    && (HartOwner(old_s, calling_hart) != SBI_IMPLEMENTATION ==> result.ret == 0)
    && (result.ret == 0 ==> !HartExecutingInSMode(new_s, calling_hart))
    && (result.ret == 0 ==> HartOwner(new_s, calling_hart) == SBI_IMPLEMENTATION)
    && (result.ret == 0 ==> !CallReturns())
}