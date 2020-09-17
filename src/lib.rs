#[cfg(test)]
mod tests;

pub mod perf;
pub mod searches;
pub mod sorts;

pub fn add_binary_integers(a: &[bool], b: &[bool]) -> Result<std::vec::Vec<bool>, &'static str> {
    match a.len().cmp(&b.len()) {
        std::cmp::Ordering::Equal => {
            let mut v = vec![];
            v.reserve(a.len());
            let mut carry = false;
            for i in 0..(a.len()) {
                let left = a[i];
                let right = b[i];
                v.insert(
                    0,
                    match carry {
                        false => (left ^ right),
                        true => !(left ^ right),
                    },
                );
                carry = left && right;
            }
            v.insert(0, carry);
            Ok(v)
        }
        _ => Err("arrays are of mismatched size"),
    }
}
