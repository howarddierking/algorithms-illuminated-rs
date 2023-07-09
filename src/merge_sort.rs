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
    let mut v1_index = 0;
    let mut v2_index = 0;

    // to handle odd-length inputs, we need to iterate for the total number of output slots
    // TODO: this is hideous - figure out the best way to clean up
    for _ in 0..v1.len() + v2.len() {
        if v1_index >= v1.len() {
            ret.push(v2[v2_index]);
            v2_index += 1;
        } else if v2_index >= v2.len() || v1[v1_index] < v2[v2_index] {
            ret.push(v1[v1_index]);
            v1_index += 1;
        } else {
            ret.push(v2[v2_index]);
            v2_index += 1;
        }
    }

    ret
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
