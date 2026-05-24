```verus
pub open spec fn rsi_vsmmu_activate_spec(result: RsiCommandReturnCode, base: Address, top: Address, new_base: Address, old_s: S, new_s: S) -> bool {
    let realm = CurrentRealm(old_s);
    let walk = RttWalk(old_s, realm, base, RMM_RTT_PAGE_LEVEL);
    let vsmmu = VsmmuAt(old_s, walk.rtte.addr);
    
    (!AddrIsGranuleAligned(base) ==> result == RSI_ERROR_INPUT)
    && (!AddrIsGranuleAligned(top) ==> result == RSI_ERROR_INPUT)
    && ((top as int) <= (base as int) ==> result == RSI_ERROR_INPUT)
    && (!AddrRangeIsProtected(base, top, realm) ==> result == RSI_ERROR_INPUT)
    && (walk.rtte.state != ASSIGNED_VSMMU ==> result == RSI_ERROR_INPUT)
    && (
        (AddrIsGranuleAligned(base) && AddrIsGranuleAligned(top) && (top as int) > (base as int)
         && AddrRangeIsProtected(base, top, realm) && walk.rtte.state == ASSIGNED_VSMMU)
        ==> (result.is_Ok()
             && RttEntriesInRangeRipas(old_s, RttAt(old_s, walk.rtte.addr), RMM_RTT_PAGE_LEVEL, base, new_base, DEV)
             && ((base == vsmmu.reg_base && new_base != vsmmu.reg_top)
                 ==> VsmmuAt(new_s, walk.rtte.addr).state == VSMMU_ACTIVATING)
             && (new_base == vsmmu.reg_top
                 ==> VsmmuAt(new_s, walk.rtte.addr).state == VSMMU_ACTIVE))
    )
}
```