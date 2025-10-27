#![forbid(unsafe_code)]
#[macro_use]
extern crate afl;

use smallvec::*;
use global_data::*;
use std::str::FromStr;
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::borrow::{Borrow, BorrowMut};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CustomType1(String);

impl core::clone::Clone for CustomType1 {
    fn clone(&self) -> Self {
        let global_data = get_global_data();
        let GLOBAL_DATA = global_data.first_half;
        let custom_impl_num = _to_usize(GLOBAL_DATA, 9);
        let custom_impl_inst_num = self.0.len();
        let selector = (custom_impl_num + custom_impl_inst_num) % 3;
        if selector == 0{
            panic!("INTENTIONAL PANIC!");
        }
        let GLOBAL_DATA = match selector{
                1 => global_data.first_half,
                _ => global_data.second_half,
        };
        let mut t_2 = _to_u8(GLOBAL_DATA, 17) % 17;
        let t_3 = _to_str(GLOBAL_DATA, 18, 18 + t_2 as usize);
        let t_4 = String::from(t_3);
        let t_5 = CustomType1(t_4);
        return t_5;
    }
}

fn main (){
    fuzz_nohook!(|data: &[u8]| {
        if data.len() < 650 {return;}
        set_global_data(data);
        let global_data = get_global_data();
        let F = global_data.first_half;
        let S = global_data.second_half;

        let la = (_to_u8(F, 30) % 32) as usize;
        let sa = 31usize;
        let ea = sa + la;
        let ca = CustomType1(String::from(_to_str(F, sa, ea)));

        let lb = (_to_u8(S, 20) % 40) as usize;
        let sb = 21usize;
        let eb = sb + lb;
        let cb = CustomType1(String::from(_to_str(S, sb, eb)));

        let lc = (_to_u8(F, 70) % 50) as usize;
        let sc = 71usize;
        let ec = sc + lc;
        let cc = CustomType1(String::from(_to_str(F, sc, ec)));

        let n1 = _to_usize(F, 90);
        let mut sv_main = smallvec::SmallVec::<[CustomType1; 16]>::from_elem(ca.clone(), n1);

        let n2 = _to_usize(S, 42);
        let mut sv_other = smallvec::SmallVec::<[CustomType1; 16]>::from_elem(cb.clone(), n2);

        let vcap = (_to_u8(F, 100) % 65) as usize;
        let mut vtmp: Vec<CustomType1> = Vec::with_capacity(vcap);
        let vcount = (_to_u8(F, 101) % 65) as usize;
        for i in 0..vcount {
            let choose = _to_u8(S, 60 + (i % 10)) % 3;
            let it = match choose {
                0 => ca.clone(),
                1 => cb.clone(),
                _ => cc.clone(),
            };
            vtmp.push(it);
        }
        let mut sv_from_vec = smallvec::SmallVec::<[CustomType1; 16]>::from_vec(vtmp);

        let slcap = (_to_u8(S, 110) % 65) as usize;
        let mut temp_vec2: Vec<CustomType1> = Vec::with_capacity(slcap);
        let cnt2 = (_to_u8(S, 111) % 65) as usize;
        for i in 0..cnt2 {
            let choose = _to_u8(F, 120 + (i % 8)) % 3;
            let it = match choose {
                0 => cb.clone(),
                1 => cc.clone(),
                _ => ca.clone(),
            };
            temp_vec2.push(it);
        }
        let sv_from_slice = smallvec::SmallVec::<[CustomType1; 16]>::from(&temp_vec2[..]);

        let cap = _to_usize(F, 130);
        let mut sv_cap = smallvec::SmallVec::<[CustomType1; 16]>::with_capacity(cap);
        sv_cap.push(ca.clone());
        sv_cap.push(cb.clone());

        let _ = core::cmp::PartialEq::eq(&sv_main, &sv_other);
        let _ = core::cmp::PartialOrd::partial_cmp(&sv_main, &sv_other);
        let _ = core::cmp::Ord::cmp(&sv_main, &sv_other);

        let sref = smallvec::SmallVec::<[CustomType1; 16]>::as_slice(&sv_other);
        if sref.len() > 0 {
            println!("{:?}", &sref[0]);
        }
        let smut = smallvec::SmallVec::<[CustomType1; 16]>::as_mut_slice(&mut sv_other);
        if smut.len() > 0 {
            println!("{:?}", &smut[0]);
        }
        let brr: &[CustomType1] = std::borrow::Borrow::borrow(&sv_from_vec);
        if brr.len() > 0 {
            println!("{:?}", &brr[0]);
        }
        let brm: &mut [CustomType1] = std::borrow::BorrowMut::borrow_mut(&mut sv_from_vec);
        if brm.len() > 0 {
            println!("{:?}", &brm[0]);
        }
        let aref = smallvec::SmallVec::<[CustomType1; 16]>::as_ref(&sv_from_slice);
        if aref.len() > 0 {
            println!("{:?}", &aref[0]);
        }
        let amut = smallvec::SmallVec::<[CustomType1; 16]>::as_mut(&mut sv_from_vec);
        if amut.len() > 0 {
            println!("{:?}", &amut[0]);
        }
        let ds: &[CustomType1] = smallvec::SmallVec::<[CustomType1; 16]>::deref(&sv_cap);
        if ds.len() > 0 {
            println!("{:?}", &ds[0]);
        }
        let dms: &mut [CustomType1] = smallvec::SmallVec::<[CustomType1; 16]>::deref_mut(&mut sv_cap);
        if dms.len() > 0 {
            println!("{:?}", &dms[0]);
        }

        let op_count = (_to_u8(S, 140) % 16) as usize;
        for i in 0..op_count {
            let sel = _to_u8(F, 150 + (i % 20));
            match sel % 16 {
                0 => {
                    sv_main.push(cc.clone());
                }
                1 => {
                    let idx = _to_usize(S, 160 + (i % 16));
                    sv_main.insert(idx, cb.clone());
                }
                2 => {
                    let idx = _to_usize(F, 170 + (i % 16));
                    if sv_main.len() > 0 {
                        let _ = smallvec::SmallVec::<[CustomType1; 16]>::remove(&mut sv_main, idx);
                    }
                }
                3 => {
                    let idx = _to_usize(S, 180 + (i % 16));
                    if sv_main.len() > 0 {
                        let _ = smallvec::SmallVec::<[CustomType1; 16]>::swap_remove(&mut sv_main, idx);
                    }
                }
                4 => {
                    let new_len = _to_usize(F, 190 + (i % 8));
                    smallvec::SmallVec::<[CustomType1; 16]>::truncate(&mut sv_main, new_len);
                }
                5 => {
                    smallvec::SmallVec::<[CustomType1; 16]>::extend(&mut sv_main, sv_other.iter().cloned());
                }
                6 => {
                    let len = _to_usize(S, 200 + (i % 8));
                    smallvec::SmallVec::<[CustomType1; 16]>::resize(&mut sv_main, len, ca.clone());
                }
                7 => {
                    let len = _to_usize(F, 210 + (i % 8));
                    smallvec::SmallVec::<[CustomType1; 16]>::resize_with(&mut sv_main, len, || {
                        let choose = _to_u8(S, 220 + (i % 8)) % 3;
                        match choose {
                            0 => ca.clone(),
                            1 => cb.clone(),
                            _ => {
                                if _to_u8(F, 230 + (i % 4)) % 2 == 0 {
                                    panic!("INTENTIONAL PANIC!");
                                }
                                cc.clone()
                            }
                        }
                    });
                }
                8 => {
                    smallvec::SmallVec::<[CustomType1; 16]>::dedup_by(&mut sv_main, |a, b| a.0.len() == b.0.len());
                }
                9 => {
                    smallvec::SmallVec::<[CustomType1; 16]>::retain(&mut sv_main, |x| {
                        let v = _to_u8(F, 240 + (i % 8));
                        if v % 3 == 0 { return true; }
                        x.0.len() % 2 == 0
                    });
                }
                10 => {
                    let start = _to_usize(S, 246);
                    let end = _to_usize(S, 254);
                    let mut dr = smallvec::SmallVec::<[CustomType1; 16]>::drain(&mut sv_main, start..end);
                    let _ = smallvec::Drain::next(&mut dr);
                    let _ = smallvec::Drain::next_back(&mut dr);
                }
                11 => {
                    smallvec::SmallVec::<[CustomType1; 16]>::append(&mut sv_main, &mut sv_other);
                }
                12 => {
                    let add = _to_usize(F, 260);
                    smallvec::SmallVec::<[CustomType1; 16]>::reserve(&mut sv_main, add);
                    let add2 = _to_usize(S, 260);
                    let _ = smallvec::SmallVec::<[CustomType1; 16]>::try_reserve(&mut sv_main, add2);
                }
                13 => {
                    let idx = _to_usize(F, 268);
                    let mut iter_src: Vec<CustomType1> = Vec::new();
                    let iter_n = (_to_u8(S, 270) % 65) as usize;
                    for _ in 0..iter_n {
                        iter_src.push(cb.clone());
                    }
                    smallvec::SmallVec::<[CustomType1; 16]>::insert_many(&mut sv_main, idx, iter_src);
                }
                14 => {
                    let grow_to = _to_usize(S, 278);
                    smallvec::SmallVec::<[CustomType1; 16]>::grow(&mut sv_main, grow_to);
                    let _ = smallvec::SmallVec::<[CustomType1; 16]>::try_grow(&mut sv_main, grow_to);
                }
                _ => {
                    let re = _to_usize(F, 286);
                    smallvec::SmallVec::<[CustomType1; 16]>::reserve_exact(&mut sv_main, re);
                    let _ = smallvec::SmallVec::<[CustomType1; 16]>::try_reserve_exact(&mut sv_main, re);
                }
            }
            let _ = smallvec::SmallVec::<[CustomType1; 16]>::from_elem(cc.clone(), _to_usize(S, 294));
        }

        if sv_main.len() > 0 {
            println!("{:?}", &sv_main[sv_main.len()-1]);
        }
        if sv_main.len() > 0 {
            println!("{:?}", &sv_main[0]);
        }

        let mut it = smallvec::SmallVec::<[CustomType1; 16]>::into_iter(sv_main.clone());
        let _ = smallvec::IntoIter::as_slice(&it);
        let _ = smallvec::IntoIter::as_mut_slice(&mut it);
        let _ = smallvec::IntoIter::next(&mut it);
        let _ = smallvec::IntoIter::next_back(&mut it);
        let _ = smallvec::IntoIter::clone(&it);

        let _ = smallvec::SmallVec::<[CustomType1; 16]>::into_boxed_slice(sv_other);
        let _ = smallvec::SmallVec::<[CustomType1; 16]>::into_vec(sv_from_slice);
        let _ = smallvec::SmallVec::<[CustomType1; 16]>::into_inner(sv_from_vec);

        let tlen = (_to_u8(F, 300) % 20) as usize;
        let ars = 301usize;
        let are = ars + tlen;
        let arr_elem = CustomType1(String::from(_to_str(F, ars, are)));
        let buf: [CustomType1; 12] = [arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone()];
        let mut sv_from_buf = smallvec::SmallVec::<[CustomType1; 12]>::from_buf(buf);
        sv_from_buf.push(arr_elem.clone());
        println!("{:?}", smallvec::SmallVec::<[CustomType1; 12]>::len(&sv_from_buf));

        let len_for_buf = _to_usize(S, 308);
        let mut sv_fbal = smallvec::SmallVec::<[CustomType1; 12]>::from_buf_and_len([arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone(), arr_elem.clone()], len_for_buf);
        smallvec::SmallVec::<[CustomType1; 12]>::clear(&mut sv_fbal);

        let val_x = CustomType1(String::from("x"));
        let ts: [CustomType1; 12] = [val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone(), val_x.clone()];
        let mut sv_from_arr = smallvec::SmallVec::<[CustomType1; 12]>::from(ts);
        smallvec::SmallVec::<[CustomType1; 12]>::shrink_to_fit(&mut sv_from_arr);

        let val_y = CustomType1(String::from("y"));
        let tsv: [CustomType1; 12] = [val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone(), val_y.clone()];
        let _ = smallvec::SmallVec::<[CustomType1; 12]>::from(&tsv[..]);

        let mut to_sv = smallvec::SmallVec::<[CustomType1; 12]>::from(&tsv[..]);
        smallvec::SmallVec::<[CustomType1; 12]>::pop(&mut to_sv);
    });
}

fn _to_u8(data:&[u8], index:usize)->u8 {
    data[index]
}

fn _to_u16(data:&[u8], index:usize)->u16 {
    let data0 = _to_u8(data, index) as u16;
    let data1 = _to_u8(data, index+1) as u16;
    data0 << 8 | data1
}

fn _to_u32(data:&[u8], index:usize)->u32 {
    let data0 = _to_u16(data, index) as u32;
    let data1 = _to_u16(data, index+2) as u32;
    data0 << 16 | data1
}

fn _to_u64(data:&[u8], index:usize)->u64 {
    let data0 = _to_u32(data, index) as u64;
    let data1 = _to_u32(data, index+4) as u64;
    data0 << 32 | data1
}

fn _to_u128(data:&[u8], index:usize)->u128 {
    let data0 = _to_u64(data, index) as u128;
    let data1 = _to_u64(data, index+8) as u128;
    data0 << 64 | data1
}

fn _to_usize(data:&[u8], index:usize)->usize {
    _to_u64(data, index) as usize
}

fn _to_i8(data:&[u8], index:usize)->i8 {    
    data[index] as i8
}

fn _to_i16(data:&[u8], index:usize)->i16 {
    let data0 = _to_i8(data, index) as i16;
    let data1 = _to_i8(data, index+1) as i16;
    data0 << 8 | data1
}

fn _to_i32(data:&[u8], index:usize)->i32 {
    let data0 = _to_i16(data, index) as i32;
    let data1 = _to_i16(data, index+2) as i32;
    data0 << 16 | data1
}

fn _to_i64(data:&[u8], index:usize)->i64 {
    let data0 = _to_i32(data, index) as i64;
    let data1 = _to_i32(data, index+4) as i64;
    data0 << 32 | data1
}

fn _to_i128(data:&[u8], index:usize)->i128 {
    let data0 = _to_i64(data, index) as i128;
    let data1 = _to_i64(data, index+8) as i128;
    data0 << 64 | data1
}

fn _to_isize(data:&[u8], index:usize)->isize {
    _to_i64(data, index) as isize
}

fn _to_f32(data:&[u8], index: usize) -> f32 {
    let data_slice = &data[index..index+4];
    use std::convert::TryInto;
    let data_array:[u8;4] = data_slice.try_into().expect("slice with incorrect length");
    f32::from_le_bytes(data_array)
}

fn _to_f64(data:&[u8], index: usize) -> f64 {
    let data_slice = &data[index..index+8];
    use std::convert::TryInto;
    let data_array:[u8;8] = data_slice.try_into().expect("slice with incorrect length");
    f64::from_le_bytes(data_array)
}

fn _to_char(data:&[u8], index: usize)->char {
    let char_value = _to_u32(data,index);
    match char::from_u32(char_value) {
        Some(c)=>c,
        None=>{
            std::process::exit(0);
        }
    }
}

fn _to_bool(data:&[u8], index: usize)->bool {
    let bool_value = _to_u8(data, index);
    if bool_value %2 == 0 {
        true
    } else {
        false
    }
}

fn _to_str(data:&[u8], start_index: usize, end_index: usize)->&str {
    let data_slice = &data[start_index..end_index];
    use std::str;
    match str::from_utf8(data_slice) {
        Ok(s)=>s,
        Err(_)=>{
            std::process::exit(0);
        }
    }
}

fn _unwrap_option<T>(opt: Option<T>) -> T {
    match opt {
        Some(_t) => _t,
        None => {
            std::process::exit(0);
        }
    }
}

fn _unwrap_result<T, E>(_res: std::result::Result<T, E>) -> T {
    match _res {
        Ok(_t) => _t,
        Err(_) => {
            std::process::exit(0);
        },
    }
}