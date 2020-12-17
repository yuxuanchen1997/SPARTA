/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

pub trait AbstractDomain {
    fn bottom() -> Self;
    fn top() -> Self;
    fn is_bottom(&self) -> bool;
    fn is_top(&self) -> bool;
    fn leq(&self, rhs: &Self) -> bool;
  
    // The following operations moves self and rhs
    fn join(self, rhs: Self) -> Self;
    fn widen(self, rhs: Self) -> Self;
    fn meet(self, rhs: Self) -> Self;
    fn narrow(self, rhs: Self) -> Self;
}
