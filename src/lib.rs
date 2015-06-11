#![feature(test)]
use std::cmp::Ordering;
use std::ops::Range;

#[inline]
fn block_swap<T>(array: &mut [T], index1: usize, index2: usize, count: usize) {
    for ind in 0..count {
        array.swap(index1+ind, index2+ind);
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
    let max_ind = (range.len()/2) -1;
    let mut ind = 0;
    while ind <= max_ind {
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

    let range1 = range.start..split;
    let range2 = split..range.end;
    reverse(array, range1);
    reverse(array, range2);
    reverse(array, range);
}

fn binary_first<T,F>(array: &[T], range: Range<usize>, value: T, compare: F) -> usize
    where F: Fn(&T, &T) -> Ordering {

    let mut start = range.start;
    let mut end = range.end - 1;
    while start < end {
        let mid = start + (end - start)/2;
        // i.e. array[mid] < value
        if compare(&array[mid], &value) == Ordering::Less {
            start = mid + 1
        } else {
            end = mid
        };

    }

    // i.e. if (start == range.end && array[start] < value)
    if start == range.end && compare(&array[start], &value) == Ordering::Less {
        start += 1;
    }
    return start;
}

fn binary_last<T,F>(array: &[T], range: Range<usize>, value: T, compare: F) -> usize
    where F: Fn(&T, &T) -> Ordering {

    let mut start = range.start;
    let mut end = range.end -1;
    while start < end {
        let mid = start + (end - start)/2;
        // i.e. array[mid] <= value
        if compare(&array[mid], &value) != Ordering::Greater {
            start = mid + 1
        } else {
            end = mid
        };

    }
    // i.e. if (start == range.end && array[start] <= value)
    if start == range.end  && compare(&array[start], &value) != Ordering::Greater {
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
                && compare(&array[(j-1)], &array[j]) == Ordering::Greater {
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
    if range.start > range.end {
        panic!("Range start must be lesser than end");
    } else if range.start == range.end {
        return;
    }
    insertion_sort_helper(array, range, &compare);
}

fn insertion_sort<T>(array: &mut[T], range: Range<usize>)
    where T: Ord {
    insertion_sort_by(array, range, |a, b| a.cmp(b));
}

fn main() {
    let mut arr = [6,2,5,4,3,1,0];
    println!("{:?}", arr);
    insertion_sort(&mut arr, (0..3));
    println!("{:?}", arr);
}


#[cfg(test)]
pub mod test {
    #![feature(test)]
    extern crate test;

    use super::{block_swap, reverse, rotate};

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


}