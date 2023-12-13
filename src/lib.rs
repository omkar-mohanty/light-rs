pub fn add(left: usize, right: usize) -> usize {
    left + right
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [9,3,2,1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1,2,3,9]);
    }
}

pub fn insertion_sort<T: Ord + Copy>(a : &mut [T]) {
    for i in 1..a.len() {
        let key = a[i];

        let mut j = i;

        while j > 0 && a[j-1] > key {
            a[j] = a[j-1];
            j -= 1;
        } 
        a[j] = key;
    }
}
