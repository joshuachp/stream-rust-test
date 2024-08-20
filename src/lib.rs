// This file is part of Astarte.
//
// Copyright 2024 SECO Mind Srl
//
// SPDX-License-Identifier: Apache-2.0

pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use crate::sum;

    #[test]
    fn one_plus_one() {
        assert_eq!(sum(1, 1), 2)
    }
}
