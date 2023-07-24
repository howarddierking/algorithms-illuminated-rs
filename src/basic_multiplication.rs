/// Rust lessons learned in this example
/// - trying to iterate in Rust using C++ indices is an invitation for pain. Iterators are clearly
///   the expected way
/// - pattern matching is really quite lovely in that you can return anything, including this tuple
///   that I'm then destructuring into 2 discrete values
/// - Rust uses `usize` for working with indices and slices, so if you want to do arithmetic
///   operations on those values as we're doing here to compute the exponent, you'll need to cast
///   to a type such as `u32`
/// - for..in consumes the iterator, making it useless inside the body of the iterator. This is
///   problematic if you need to use a feature like `peek`. Instead, use the while loop, which it
///   turns out is what for..in de-sugars to anyway
///
pub fn multiply(x: i32, y: i32) -> i32 {
    let (mut smaller_val_items, mut larger_val_items) = match x <= y {
        true => { (get_int_vector(x), get_int_vector(y))}
        false => { (get_int_vector(y), get_int_vector(x))}
    };

    // reverse the vectors
    smaller_val_items.reverse();
    larger_val_items.reverse();

    // create the vector iterators
    let smaller_item_iter = smaller_val_items.into_iter().peekable().enumerate();
    let larger_item_iter = larger_val_items.into_iter().enumerate().peekable();

    // initialize a total
    let mut total = 0;

    // for each value of the shorter vector
    for (smaller_pos, smaller_val) in smaller_item_iter {
        let mut carry_over = 0;
        let mut sub = 0;

        let mut larger_item_iter_clone = larger_item_iter.clone();
        while let Some(larger_item) = larger_item_iter_clone.next() {
            let (larger_pos, larger_val) = larger_item;
            // get the value of place value multiplication and add in any carry-overs
            let intermediate = (smaller_val * larger_val) + carry_over;


            // if the value > 10, split the 1s from the 10s
            let ones = intermediate % 10;
            let tens = intermediate / 10;

            // update the carry-over
            carry_over = tens;

            // add to sub the value of the 1s place value * 10^(i+j) - this is what enables us to
            // build up the subs correctly even though we're reading the input right to left
            let a: u32 = smaller_pos.try_into().unwrap();
            let b: u32 =  larger_pos.try_into().unwrap();
            let decimal_offset = a+b;

            sub += ones * 10i32.pow(decimal_offset);

            // if we're at the end of this iter, sub += carry_over * 10^(decimal_offset + 1)
            if larger_item_iter_clone.peek().is_none() {
                sub += carry_over * 10i32.pow(decimal_offset + 1)
            }
        }

        total += sub;
    }

    total
}

fn get_int_vector(i: i32) -> Vec<i32> {
    i.to_string()
        .chars()
        .map(|c| -> i32 { c.to_digit(10).unwrap().try_into().unwrap() })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{get_int_vector, multiply};

    #[test]
    fn integer_to_vector_of_integers() {
        assert_eq!(vec![1, 2, 3], get_int_vector(123));
    }

    #[test]
    fn multiplication_produces_expected_outcome() {
        assert_eq!(45 * 94, multiply(45, 94));
        assert_eq!(234 * 10492, multiply(234, 10492));
    }
}
