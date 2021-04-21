use std::slice;

fn split_at_mut(slice_arr: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let get_len = slice_arr.len();
    let ptr = slice_arr.as_mut_ptr();
    assert!(mid <= get_len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), get_len - mid),
        )
    }
}

fn main() {
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
    println!("left = {:?}", left);
    println!("right = {:?}", right);

    // creating a slice from arbitrary memory location
    let address = 0x01234usize;
    let r_add = address as *mut i32;
    let slice_var: &[i32] = unsafe { slice::from_raw_parts_mut(r_add, 10000) };
    println!("slice_var = {:?}", slice_var);
}
