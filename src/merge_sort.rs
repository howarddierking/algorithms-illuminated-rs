/// merge_sort
/// * classic divide and conquer approach - break problem into sub-problems, solve recursively,
/// and then combine the solutions
/// * select sort, insertion sort, and bubble sort all have quadratic running times
/// If I'm getting the concept correctly, the real running time improvement is in the merge aspect
/// of the algorithm. It seems like you still have to effectively process the entire vector through
/// recursive application, but you combine the results back together in a way that keeps you from
/// having to go back through every other value
pub fn merge_sort(mut input: Vec<i32>) -> Vec<i32> {
    if input.len() < 2 {
        return input;
    }

    let split = input.split_off(input.len() / 2);

    // recurse with both sides
    let a = merge_sort(input);
    let b = merge_sort(split);

    merge(a, b)
}

fn merge(v1: Vec<i32>, v2: Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();

    let mut v1 = v1.into_iter().peekable();
    let mut v2 = v2.into_iter().peekable();

    loop {
        // Inspect the next element of each list without modifying the iterator
        match (v1.peek(), v2.peek()) {
            // a < b, so push a while moving to the next element in v1
            (Some(a), Some(b)) if a < b => ret.push(v1.next().unwrap()),
            // b < a, so push b while moving to the next element in v2
            (Some(a), Some(b)) if b <= a => ret.push(v2.next().unwrap()),
            // v2 is empty, so push the last element(s) of v1
            (Some(_), None) => ret.push(v1.next().unwrap()),
            // v1 is empty, so push the last element(s) of v2
            (None, Some(_)) => ret.push(v2.next().unwrap()),
            // both lists are empty, so return
            _ => return ret,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_8_ints() {
        let v = vec![2, 4, 3, 5, 8, 6, 7, 1];
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], merge_sort(v));
    }

    #[test]
    fn sort_7_ints() {
        let v = vec![2, 4, 3, 5, 6, 7, 1];
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7], merge_sort(v));
    }

    #[test]
    fn already_sorted() {
        let v = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], merge_sort(v));
    }

    #[test]
    fn worst_case() {
        let v = vec![8, 7, 6, 5, 4, 3, 2, 1];
        assert_eq!(vec![1, 2, 3, 4, 5, 6, 7, 8], merge_sort(v));
    }
}
