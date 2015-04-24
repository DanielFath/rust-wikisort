use std::cmp::Ordering;
use std::ptr::swap;
use std::ops::Range;


fn block_swap<T>(array: &mut [T], index1: usize, index2: usize, count: usize) {
    for ind in 0..count {
        array.swap(index1+ind, index2+ind);
    }
}

fn reverse<T>(array: &mut [T], range: Range<i64>) {
    let mut ind = ((range.end-range.start)/ 2) -1;
    loop{
        if ind < 0 {
            break;
        }

        let ind_a = (range.start + ind) as usize;
        let ind_b = (range.end - ind - 1) as usize;
        array.swap(ind_a, ind_b);

        ind -= 1;
    }
}

fn rotate<T>(array: &mut [T], range: Range<i64>, amount: isize) {

    let split = if amount >= 0 {
        range.start + amount as i64
    } else {
        range.end + amount as i64
    };

    let range1 = range.start..split;
    let range2 = split..range.end;
    reverse(array, range1);
    reverse(array, range2);
    reverse(array, range);
}

fn binary_first<T,F>(array: &mut [T], range: Range<i64>, value: T, compare: &F) -> usize
    where F: Fn(&T, &T) -> Ordering {

    let mut start = range.start;
    let mut end = range.end;
    while start < end {
        let mid = start + (end - start)/2;
        if compare(&array[mid as usize], &value) != Ordering::Less {
            start = mid + 1
        } else {
            end = mid
        };

    }
    if start == range.end
        && compare(&array[start as usize], &value) == Ordering::Less {
        start += 1;
    }
    return start as usize;
}

fn binary_last<T,F>(array: &mut [T], range: Range<i64>, value: T, compare: &F) -> usize
    where F: Fn(&T, &T) -> Ordering {

    let mut start = range.start;
    let mut end = range.end;
    while start < end {
        let mid = start + (end - start)/2;
        if compare(&array[mid as usize], &value) != Ordering::Greater {
            start = mid + 1
        } else {
            end = mid
        };

    }
    if start == range.end
             && compare(&array[start as usize], &value) != Ordering::Greater {
        start += 1;
    }
    return start as usize;
}

fn insertion_sort_helper<T,F>(array: &mut [T], range: Range<i64>, compare: &F)
    where F: Fn(&T, &T) -> Ordering {

    let (mut i, len) = (range.start +1, range.end);
    while i < len {
        let mut j = i;
        while j > range.start
                && compare(&array[(j-1) as usize], &array[j as usize]) == Ordering::Greater {
            array.swap(j as usize, (j-1) as usize);
            j -= 1;
        }
        i += 1;
    }
}

fn insertion_sort_by<T,F>(array: &mut [T], range: Range<i64>, compare: F)
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

fn insertion_sort<T>(array: &mut[T], range: Range<i64>)
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
    use super::{block_swap, reverse, rotate};

    #[test]
    fn test_blockswap() {
        let mut arr = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14];
        block_swap(&mut arr, 0, 10, 5);
        let swaped = [10,11,12,13,14,5,6,7,8,9,0,1,2,3,4];
        assert_eq!(arr, swaped);
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