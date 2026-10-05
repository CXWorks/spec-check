pub open spec fn sbi_system_suspend_spec(result: SbiErrorCode, old_s: S, new_s: S) -> bool {
    (!HartResumedFrom(old_s, STOPPED) ==> result != SBI_SUCCESS)
    && (HartResumedFrom(old_s, STOPPED) ==> result == SBI_SUCCESS)
    && (result == SBI_SUCCESS ==> CurrentPrivilegeMode(new_s) == SUPERVISOR)
    && (result == SBI_SUCCESS ==> pc(new_s) == old_s.resume_addr)
    && (result == SBI_SUCCESS ==> satp(new_s) == 0)
    && (result == SBI_SUCCESS ==> sstatus(new_s).SIE == 0)
    && (result == SBI_SUCCESS ==> a0(new_s) == old_s.hartid)
    && (result == SBI_SUCCESS ==> a1(new_s) == old_s.opaque)
}