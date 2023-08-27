
/// mark code path as cold.
#[inline(always)]
#[cold]
pub fn cold() {}

/// mark condition as likely.
#[inline(always)]
pub fn likely(cond: bool) -> bool {
    if !cond { cold() }
    return cond;
}

/// mark condition as unlikely.
#[inline(always)]
pub fn unlikely(cond: bool) -> bool {
    if cond { cold() }
    return cond;
}

