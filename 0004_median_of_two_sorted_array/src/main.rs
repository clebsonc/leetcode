pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut sorted_array = Vec::<i32>::with_capacity(nums1.len() + nums2.len());

    let m1 = nums1.len();
    let m2 = nums2.len();

    let mut i = 0;
    let mut j = 0;

    while i < m1 && j < m2 {
        if nums1[i] <= nums2[j] {
            sorted_array.push(*nums1.get(i).unwrap());
            i += 1;
        } else {
            sorted_array.push(*nums2.get(j).unwrap());
            j += 1;
        }
    }
    for u in i..m1 {
        sorted_array.push(*nums1.get(u).unwrap());
    }
    for u in j..m2 {
        sorted_array.push(*nums2.get(u).unwrap());
    }

    let middle = sorted_array.len() / 2;
    if sorted_array.len() % 2 == 0 {
        return (sorted_array[middle - 1] + sorted_array[middle]) as f64 / 2.0;
    }
    sorted_array[middle] as f64
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_find_median_sorted_arrays() {
        assert_eq!(find_median_sorted_arrays(vec![1, 3], vec![2]), 2.0);
        assert_eq!(find_median_sorted_arrays(vec![1, 2], vec![3, 4]), 2.5);
    }
}

fn main() {
    println!("Hello, world!");
}
