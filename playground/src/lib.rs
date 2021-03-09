extern crate prusti_contracts;
use prusti_contracts::*;

#[pure]
fn is_okay(opt: &Option<u64>) -> bool {
    return true;
}

#[ensures(is_okay(opt))]
fn test_ok(opt: Option<u64>) {
    return;
}

// #[ensures(
//     match result {
//         None => true, 
//         Some(x) => contains(vec, x)
//     })]
// #[ensures(ok(result))]
fn max(vec: &Vec<u64>) -> Option<u64> {
    let len = veclen(vec);
    if(len > 0) {
        let mut max = get_and_unwrap(vec, 0);
        let mut index = 1;
        while (index < veclen(vec)) {
            body_invariant!(index < veclen(&vec));
            let candidate = get_and_unwrap(vec, index);
            if(candidate > max) {
                max = candidate;
            }
            index += 1;
        }
        return Some(max)
    } else {
        return None;
    }
}

#[pure]
#[trusted]
fn veclen(vec: &Vec<u64>) -> usize {
    return vec.len();
}

#[pure]
fn contains(vec: &Vec<u64>, element: u64) -> bool {
    return contains_helper(element, vec, 0);
}

#[pure]
fn contains_helper(needle: u64, haystack: &Vec<u64>, at: usize) -> bool {
    if at < veclen(&haystack) {
        let e = get_and_unwrap(&haystack, at);
        if (e == needle) {
            return true;
        } else {
            return contains_helper(needle, haystack, at + 1);
        }
    } else {
        return false;
    }
}

#[trusted]
#[pure]
#[requires(index < veclen(vec))]
fn get_and_unwrap(vec: &Vec<u64>, index: usize) -> u64 {
    return vec.get(index).unwrap().clone();
}