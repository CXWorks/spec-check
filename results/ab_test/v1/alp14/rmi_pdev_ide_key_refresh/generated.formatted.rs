pub open spec fn RMI_PDEV_IDE_KEY_REFRESH_spec(
    s: S,
    pdev_ptr: Address,
    coh: RmiPdevCoherent,
) -> (result: Result<(), RmiStatusCode>, s_post: S)
{
    // Failure condition: da_supp
    if !ImplFeatures().feat_da {
        (Err(RMI_ERROR_NOT_SUPPORTED), s)
    }
    // Failure condition: pdev_align
    else if !AddrIsGranuleAligned(pdev_ptr) {
        (Err(RMI_ERROR_INPUT), s)
    }
    // Failure condition: pdev_bound
    else if !PaIsDelegable(pdev_ptr) {
        (Err(RMI_ERROR_INPUT), s)
    }
    // Failure condition: pdev_gran_state
    else if GranuleAt(pdev_ptr).state != PDEV {
        (Err(RMI_ERROR_INPUT), s)
    }
    // Failure condition: no_connection
    else if ((coh == RMI_NCOH && PdevAt(s, pdev_ptr).ncoh_ide != IDE_TRUE)
             || (coh == RMI_COH && PdevAt(s, pdev_ptr).coh_ide != IDE_TRUE)) {
        (Err(RMI_ERROR_DEVICE), s)
    }
    // Failure condition: pdev_state
    else if PdevAt(s, pdev_ptr).state != PDEV_READY {
        (Err(RMI_ERROR_DEVICE), s)
    }
    // Success conditions
    else {
        let pdev = PdevAt(s, pdev_ptr);
        let pdev_updated = RmmPdev {
            state: PDEV_COMMUNICATING,
            comm_state: DEV_COMM_PENDING,
            ..pdev
        };
        let s_post = s.update_pdev(pdev_ptr, pdev_updated);
        (Ok(()), s_post)
    }
}