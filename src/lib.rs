#![feature(test)]
use std::cmp::Ordering;
use std::cmp::Ordering::*;
use std::ops::Range;

#[inline]
fn block_swap<T>(array: &mut [T], index1: usize, index2: usize, count: usize) {
    for ind in 0..count {
        array.swap(index1+ind, index2+ind);
    }
}

fn array_copy<T>(src:&[T], src_pos: usize, dst: &mut[T], dst_pos: usize, len: usize) {
    use std::ptr;

    assert!(src_pos+len < src.len());
    assert!(dst_pos+len < dst.len());

    unsafe {
        // TODO check if it works
        ptr::copy(&src[src_pos], &mut dst[dst_pos], len);
    }
}

fn reverse<T>(array: &mut [T], range: Range<usize>) {
    if range.len() < 2 {
        // We don't want max_ind to overflow, plus
        // reversing a 0 or 1 range is pointless.
        return
    }
    // I reversed the formula for reversing so it doesn't requires casting to and
    // from unsigned to signed and back again.
    let mut ind = 0;
    while ind <= (range.len()/2) -1 {
        let ind_a = range.start + ind;
        let ind_b = range.end - ind -1;
        array.swap(ind_a, ind_b);

        ind += 1;
    }
}

fn rotate<T>(array: &mut [T], range: Range<usize>, amount: isize) {

    let split = if amount >= 0 {
        range.start + (amount as usize)
    } else {
        range.end - (amount.abs() as usize)
    };

    assert!(range.start < split && split < range.end);
    let range1 = range.start..split;
    let range2 = split..range.end;
    reverse(array, range1);
    reverse(array, range2);
    reverse(array, range);
}

#[inline]
fn linear_search<T>(array: &mut [T], range: Range<usize>, needle: T) -> Option<usize>
    where T: PartialEq + PartialOrd {

    for index in range {
        if array[index] == needle {
            return Some(index);
        }
    }
    None
}

fn binary_first<T,F>(array: &[T], range: Range<usize>, value: T, compare: F) -> usize
    where F: Fn(&T, &T) -> Ordering {

    let mut start = range.start;
    let mut end = range.end - 1;
    while start < end {
        let mid = start + (end - start)/2;
        // i.e. array[mid] < value
        if compare(&array[mid], &value) == Less {
            start = mid + 1
        } else {
            end = mid
        };

    }

    // i.e. if (start == range.end && array[start] < value)
    if start == range.end && compare(&array[start], &value) == Less {
        start += 1;
    }
    return start;
}

fn floor_power_of_two(value: usize) -> usize{
    let mut x = value;
    x |= x >> 1;
    x |= x >> 2;
    x |= x >> 4;
    x |= x >> 8;
    x |= x >> 16;
    x - (x >> 1)
}

fn binary_last<T,F>(array: &[T], range: Range<usize>, value: T, compare: F) -> usize
    where F: Fn(&T, &T) -> Ordering {

    let mut start = range.start;
    let mut end = range.end -1;
    while start < end {
        let mid = start + (end - start)/2;
        // i.e. array[mid] <= value
        if compare(&array[mid], &value) != Greater {
            start = mid + 1
        } else {
            end = mid
        };

    }
    // i.e. if (start == range.end && array[start] <= value)
    if start == range.end  && compare(&array[start], &value) != Greater {
        start += 1;
    }
    return start;
}

fn insertion_sort_helper<T,F>(array: &mut [T], range: Range<usize>, compare: &F)
    where F: Fn(&T, &T) -> Ordering {

    let (mut i, len) = (range.start +1, range.end);
    while i < len {
        let mut j = i;
        while j > range.start
                && compare(&array[(j-1)], &array[j]) == Greater {
            array.swap(j, (j-1));
            j -= 1;
        }
        i += 1;
    }
}

fn insertion_sort_by<T,F>(array: &mut [T], range: Range<usize>, compare: F)
    where F: Fn(&T, &T) -> Ordering {

    if array.len() <= 1 {
        return;
    }
    // We only sort a regular range
    if range.start >= range.end {
        return;
    }
    insertion_sort_helper(array, range, &compare);
}

fn insertion_sort<T>(array: &mut[T], range: Range<usize>)
    where T: Ord {

    insertion_sort_by(array, range, |a, b| a.cmp(b));
}

// We use helper to avoid infinite recursion in types
fn merge_sort_helper<T,F>(array: &mut [T], range: Range<usize>, compare: F)
    where F: Fn(&T, &T) -> Ordering, T: Debug {

    let mut temp_vec = unsafe {
        // Allocate and set capacity
        let mut temp = Vec::with_capacity(array.len());
        temp.set_len(array.len());
        temp
    };

    merge_sort_by(array, range, &compare, &mut temp_vec);
}

fn merge_sort_by<T,F>(array: &mut [T], range: Range<usize>, compare: &F, buf: &mut[T])
    where F: Fn(&T, &T) -> Ordering, T: Debug {


    if range.len() == 2 {
        if compare(&array[range.start], &array[range.end-1]) == Less {
            array.swap(range.start, range.end);
        }
    } else if range.len() > 2 {
        let mid = range.start + (range.end-range.start)/2;
        merge_sort_by(array,(range.start..mid), compare, buf);
        merge_sort_by(array,(mid..range.end), compare, buf);
        merge(array, range.start..mid, mid..range.end, compare, buf);
    }
}

use std::fmt::Debug;

fn print_array<T: Debug>(array: &[T]) {
    print!("[");
    for i in 0..array.len() {
        print!("{:?}, ", array[i]);
    }
    print!("]\n");
}

#[inline]
fn merge<T,F>(array: &mut [T], a: Range<usize>, b: Range<usize>, compare: F, buf: &mut[T])
    where F: Fn(&T, &T) -> Ordering, T:Debug {

    use std::ptr;

    println!("Range a: {:?}", a);
    println!("Range b: {:?}", b);

    let mut a_count = 0;
    let mut b_count = 0;
    let mut insert = 0;

    if a.len() == 0 || b.len() == 0 {
        return;
    }


    // Copy values from A into buffer
    unsafe {
   //     ptr::copy_nonoverlapping(&array[a.start], buf, array.len());
    }

    while a_count < a.len() && b_count < b.len() {
        
        print_array(&array);
        print_array(&buf);

        // if (buffer[a_count] <= array[b.start + b_count])

        if compare(&buf[a_count], &array[b.start + b_count]) != Greater {
            unsafe {
                ptr::copy_nonoverlapping(&buf[a_count], &mut array[a.start+insert], 1);
            }
            a_count += 1;
        } else {
            unsafe {
                ptr::copy_nonoverlapping(&array[b.start + b_count], &mut array[a.start+insert], 1);
            }
            b_count += 1;
        }
        insert += 1;
    }
    // Copy values from B into A
    unsafe {
        ptr::copy_nonoverlapping(&buf[a_count], &mut array[a.start + insert], a.len() - a_count);
    }

}

fn merge_sort<T>(array: &mut[T], range: Range<usize>)
    where T: Ord+Debug+Clone {


    //merge_sort_helper(array, range, |a, b| a.cmp(b));
}

fn main() {
    let mut arr = &[6i32,2,5,4];

    let mut temp_vec: Vec<i32> = Vec::with_capacity(4);
    let mut buf_dat = temp_vec.as_mut_ptr();
    let mut buf_tmp = unsafe { buf_dat.offset(4)};

    //array_copy(arr, 0, cp, 0, 2);

    unsafe {
        use std::ptr;
        use std::slice;

        println!("{:?}", arr);
        //ptr::copy(arr.as_ptr().offset(0), buf_dat.offset(0), 4);
        //ptr::copy_nonoverlapping(arr.as_ptr().offset(0), buf_dat.offset(0), 4);
        //temp_vec.set_len(4);
        let mut arr2 = slice::from_raw_parts(buf_dat.offset(0), 4);
        println!("{:?}", temp_vec);
        println!("{:?}", buf_dat);
        println!("{:?}", arr2);

    }
    //println!("{:?}", buf_dat.size);
    /*
    println!("{:?}", arr);
    merge_sort(&mut arr, 0..3);
    //test_type(|a,b| a.cmp(b), &mut arr);
    println!("{:?}", arr);
    let range = 0..1;
    println!("range:{:?}\nstart:{:?}\nend:{:?}\nlen:{:?}", range, range.start, range.end, range.len());
*/
}


#[cfg(test)]
pub mod test {
    #![feature(test)]
    extern crate test;

    use std::ops::Range;
    use std::cmp::Ord;
    use super::{
        block_swap, reverse, rotate, binary_first, binary_last,floor_power_of_two
    };

    #[test]
    fn test_blockswap() {
        let mut arr = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14];
        block_swap(&mut arr, 0, 10, 5);
        let swaped = [10,11,12,13,14,5,6,7,8,9,0,1,2,3,4];
        assert_eq!(arr, swaped);
    }

    #[bench]
    fn bench_blockswap(b: &mut test::Bencher) {
        let mut arr = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14];
        b.iter(|| {
            let n = test::black_box(1000);
            for _ in 0..n {
                block_swap(&mut arr, 0, 10, 5);
            }
        });
    }

    #[test]
    fn test_reverse() {
        let mut arr1 = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14];
        reverse(&mut arr1, (0..5));
        let swaped1 = [4,3,2,1,0,5,6,7,8,9,10,11,12,13,14];
        assert_eq!(arr1, swaped1);

        let mut arr2 = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14];
        reverse(&mut arr2, (4..8));
        let swaped2 = [0,1,2,3,7,6,5,4,8,9,10,11,12,13,14];
        assert_eq!(arr2, swaped2);
    }


    #[bench]
    fn bench_reverse(b: &mut test::Bencher) {
        let mut arr = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14];
        b.iter(|| {
            let n = test::black_box(1000);
            for _ in 0..n {
                reverse(&mut arr, (0..13));
            };
        });
    }

    #[bench]
    fn bench_reverse_native(b: &mut test::Bencher) {
        let mut arr = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14];
        b.iter(|| {
            let n = test::black_box(1000);
            for _ in 0..n {
                arr.reverse();
            };
        });
    }

    #[test]
    fn test_rotate() {
        let mut arr1 = [0,1,2,3,4];
        rotate(&mut arr1, (0..5), 2);
        let swaped1 = [2,3,4,0,1];
        assert_eq!(arr1, swaped1);

        let mut arr2 = [0,1,2,3,4,5,6];
        rotate(&mut arr2, (2..5), 1);
        let swaped2 = [0,1,3,4,2,5,6];
        assert_eq!(arr2, swaped2);

        let mut arr2 = [0,1,2,3,4,5,6];
        rotate(&mut arr2, (2..5), -1);
        let swaped2 = [0,1,4,2,3,5,6];
        assert_eq!(arr2, swaped2);
    }

    fn bin_first<T>(array: &[T], range: Range<usize>, value: T) -> usize
        where T: Ord {
        binary_first(array, range, value, |a, b| a.cmp(b))
    }

    #[test]
    fn test_bin_first() {
        let find = [0, 1, 2, 3, 4];
        assert_eq!(bin_first(&find, (1..4), 2), 2);
        let multiple = [0, 2, 2, 3, 4];
        assert_eq!(bin_first(&multiple, (1..4), 2), 1);
        let not_found = [0, 2, 2, 3, 4];
        assert_eq!(bin_first(&multiple, (1..4), -3), 1);
    }

    fn bin_last<T>(array: &[T], range: Range<usize>, value: T) -> usize
        where T: Ord {
        binary_last(array, range, value, |a, b| a.cmp(b))
    }

    #[test]
    fn test_bin_last() {
        let find = [0, 1, 2, 3, 4];
        assert_eq!(bin_last(&find, (1..4), 2), 3);
        let multiple = [0, 2, 2, 3, 4];
        assert_eq!(bin_last(&multiple, (1..4), 2), 3);
        let not_found = [0, 2, 2, 3, 4];
        assert_eq!(bin_last(&multiple, (1..4), -3), 1);
    }

    #[test]
    fn test_floor() {
        // Add code here
        assert_eq!(floor_power_of_two(1), 1);
        assert_eq!(floor_power_of_two(2), 2);
        assert_eq!(floor_power_of_two(3), 2);
        assert_eq!(floor_power_of_two(5), 4);
        assert_eq!(floor_power_of_two(63), 32);
        assert_eq!(floor_power_of_two(64), 64);
    }

}
