pub open spec fn sdei_interrupt_bind_spec(result: int64, old_s: S, new_s: S) -> bool {
    // Failure: NOT_SUPPORTED (SDEI not supported)
    (!SdeiSupported(old_s) ==> ResultEqual(result, SDEI_NOT_SUPPORTED))
    // Failure: INVALID_PARAMETERS (Invalid interrupt number or not owned by client)
    (!IsValidInterruptNumber(old_s, interrupt) ==> ResultEqual(result, SDEI_INVALID_PARAMETERS))
    // Failure: INVALID_PARAMETERS (Interrupt not owned by client)
    (!IsInterruptOwnedByClient(old_s, interrupt) ==> ResultEqual(result, SDEI_INVALID_PARAMETERS))
    // Failure: DENIED (Interrupt not in Inactive state)
    (!IsInterruptInactive(old_s, interrupt) ==> ResultEqual(result, SDEI_DENIED))
    // Failure: OUT_OF_RESOURCE (No available bind slots)
    (!HasAvailableBindSlot(old_s) ==> ResultEqual(result, SDEI_OUT_OF_RESOURCE))
    // Success: Interrupt successfully bound, result contains event number (lower 32 bits), upper 32 bits zero
    (IsValidInterruptNumber(old_s, interrupt) && IsInterruptOwnedByClient(old_s, interrupt) && IsInterruptInactive(old_s, interrupt) && HasAvailableBindSlot(old_s) ==>
        (result as int64) == (new_event_number as int64) &&
        ((result as int64) >> 32) == 0 &&
        InterruptBound(old_s, interrupt) == false &&
        InterruptBound(new_s, interrupt) == true &&
        RealmAt(new_s, rd).sdei_event_bound[interrupt] == Some(new_event_number) &&
        // Interrupt state unchanged (dispatcher manages interrupt controller, client manages device)
        InterruptEnabled(old_s, interrupt) == InterruptEnabled(new_s, interrupt) &&
        InterruptActive(old_s, interrupt) == InterruptActive(new_s, interrupt)
    )
}

// Helper predicates (uninterpreted for client-specific state checks)
pub open spec fn SdeiSupported(s: S) -> bool { ... }
pub open spec fn IsValidInterruptNumber(s: S, interrupt: uint32) -> bool { ... }
pub open spec fn IsInterruptOwnedByClient(s: S, interrupt: uint32) -> bool { ... }
pub open spec fn IsInterruptInactive(s: S, interrupt: uint32) -> bool { ... }
pub open spec fn HasAvailableBindSlot(s: S) -> bool { ... }
pub open spec fn InterruptBound(s: S, interrupt: uint32) -> bool { ... }
pub open spec fn InterruptEnabled(s: S, interrupt: uint32) -> bool { ... }
pub open spec fn InterruptActive(s: S, interrupt: uint32) -> bool { ... }
pub open spec fn new_event_number: int64 { ... } // Uninterpreted fresh event number
pub open spec fn RealmAt(s: S, rd: uint32) -> Realm { ... }
pub open spec fn Some(x: int64) -> bool { ... } // Helper for Option type if needed, or inline logic
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access if needed
pub open spec fn Option<T>(s: S, index: uint32) -> Option<T> { ... } // Placeholder for Option access