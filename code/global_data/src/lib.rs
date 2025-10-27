pub struct GlobalPtrs {
    pub first:  *const [u8],
    pub second: *const [u8],
}

pub struct GlobalData<'a> {
    pub first_half: &'a [u8],
    pub second_half: &'a [u8],
}

// initialize to two empty slices
static mut GLOBAL_DATA: GlobalPtrs = GlobalPtrs {
    first:  &[] as *const [u8],
    second: &[] as *const [u8],
};

/// Split `data` in two halves and initialize the (global) raw pointers
#[inline(always)]
pub fn set_global_data(data: &[u8]) {
    let half = data.len() / 2;
    let (first, second) = data.split_at(half);
    unsafe {
        GLOBAL_DATA.first  = first  as *const [u8];
        GLOBAL_DATA.second = second as *const [u8];
    }
}

/// Get the data that the raw pointers point to through references
#[inline(always)]
pub fn get_global_data<'a>() -> GlobalData<'a> {
    unsafe {
        // &*ptr “dereferences” the raw pointer into a `&[u8]`
        let first  = &*GLOBAL_DATA.first;
        let second = &*GLOBAL_DATA.second;
        GlobalData { first_half: first, second_half: second }
    }
}