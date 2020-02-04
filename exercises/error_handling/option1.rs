// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)

pub fn pop_too_much() -> bool { //unwrap => match
    let mut list = vec![3];

    match list.pop() {
        Some(last) => println!("The last item in the list is {:?}", last),
        None => println!("vec is empty")
    }
    
    match list.pop() {
        Some(second_to_last) => println!("The second-to-last item in the list is {:?}",second_to_last),
        None => println!("vec is empty")
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
