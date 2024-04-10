/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T: PartialOrd>(array: &mut [T]){
    if array.len() <= 1 {
        return;
    }
    let pivot = array.len() / 2;
    let mut left = 0;
    let mut right = array.len() - 1;
    array.swap(pivot, right);
    for i in 0..array.len() {
        if array[i] < array[right] {
            array.swap(i, left);
            left += 1;
        }
    }
    array.swap(left, right);
    sort(&mut array[0..left]);
    sort(&mut array[left+1..]);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}