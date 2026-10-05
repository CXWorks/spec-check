pub open spec fn sdei_event_complete_and_resume_spec(result: Int64, resume_addr: Address, old_s: S, new_s: S) -> bool {
    (!IsSdeiSupported(old_s) ==> ResultEqual(result, NOT_SUPPORTED))
    && (IsIdentifiablyInvalidResumeAddress(old_s, resume_addr) ==> ResultEqual(result, INVALID_PARAMETERS))
    && (!HandlerRunning(old_s, CallingPe(old_s)) ==> ResultEqual(result, DENIED))
    && ((IsSdeiSupported(old_s)
        && !IsIdentifiablyInvalidResumeAddress(old_s, resume_addr)
        && HandlerRunning(old_s, CallingPe(old_s)))
        ==> (ResumeContextExceptionReturnAddress(new_s) == InterruptedPc(old_s)
            && !HandlerRunning(new_s, CallingPe(old_s))
            && (IsPrivateEvent(old_s, CurrentEvent(old_s)) ==> EventHandlingComplete(new_s, CurrentEvent(old_s), CallingPe(old_s)))
            && (IsSharedEvent(old_s, CurrentEvent(old_s)) ==> EventHandlingCompleteGlobally(new_s, CurrentEvent(old_s)))))
}
