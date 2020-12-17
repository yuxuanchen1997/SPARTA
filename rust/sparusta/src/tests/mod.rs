/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

#[cfg(test)]
mod tests {
  pub use crate::ds::*;
  #[test]
  fn test_const_domain() {
    type ConstDomain = SimpleLattice::<i64>;
    let top = ConstDomain::Top;
    let value = ConstDomain::Value(42);
    
    assert!(top.is_top());
    assert!(!value.is_top());
    assert!(value.leq(&top));
    assert!(ConstDomain::Bottom.leq(&top));
  }
}