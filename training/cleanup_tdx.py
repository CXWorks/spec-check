#!/usr/bin/env python3
"""
cleanup_tdx.py

Post-process tdx_generated.rs to fix model output artifacts and produce
a syntactically valid tdx_generated_clean.rs.

Delegates all heavy lifting to cleanup_base.run_cleanup().
"""

import os
from cleanup_base import run_cleanup

BASE_DIR = os.path.dirname(os.path.abspath(__file__))

COMMANDS = sorted([
    # TDH host-side leaf functions (Chapter 14)
    "TDH_MNG_CREATE",
    "TDH_MNG_ADDCX",
    "TDH_MNG_INIT",
    "TDH_MNG_KEY_CONFIG",
    "TDH_MNG_KEY_FREEID",
    "TDH_MNG_KEY_PROMOTE",
    "TDH_VP_CREATE",
    "TDH_VP_ADDCX",
    "TDH_VP_INIT",
    "TDH_VP_ENTER",
    "TDH_VP_WR",
    "TDH_VP_RD",
    "TDH_MEM_PAGE_ADD",
    "TDH_MEM_PAGE_AUG",
    "TDH_MEM_PAGE_DEMOTE",
    "TDH_MEM_PAGE_PROMOTE",
    "TDH_MEM_PAGE_REMOVE",
    "TDH_MEM_RD",
    "TDH_MEM_WR",
    "TDH_MEM_RANGE_BLOCK",
    "TDH_MEM_RANGE_UNBLOCK",
    "TDH_MEM_SEPT_RD",
    "TDH_MEM_SEPT_REMOVE",
    "TDH_MR_EXTEND",
    "TDH_MR_FINALIZE",
    "TDH_PHYMEM_PAGE_WBINVD",
    "TDH_SYS_INIT",
    "TDH_SYS_LP_INIT",
    "TDH_SYS_CONFIG",
    # TDG guest-side leaf functions (Chapter 15)
    "TDG_VP_VMCALL",
    "TDG_VP_INFO",
    "TDG_MR_RTMR_EXTEND",
    "TDG_MR_REPORT",
    "TDG_MEM_PAGE_ATTR_WR",
    "TDG_MEM_PAGE_ATTR_RD",
    "TDG_MR_VERIFY_REPORT",
    "TDG_SERVTD_RD",
    "TDG_SERVTD_WR",
    "TDG_SERVTD_BIND",
])

if __name__ == "__main__":
    run_cleanup(
        layer1_path=os.path.join(BASE_DIR, "boilerplate", "layer1_tdx.rs"),
        generated_path=os.path.join(BASE_DIR, "tdx_generated.rs"),
        output_path=os.path.join(BASE_DIR, "tdx_generated_clean.rs"),
        commands=COMMANDS,
        extra_ops=r"\bTDX_|\bTdIs|\bTdExists|\bVcpuIs|\bVcpuExists|\bPageIs|\bPageType"
                  r"|\bKeyIs|\bGpaIs|\bHpaIs|\bTDH_|\bTDG_",
    )
