/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use super::abstract_domain::AbstractDomain;

#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum SimpleLattice<T: PartialEq + Clone> {
  Top,
  Value(T),
  Bottom,
}

impl<T: PartialEq + Clone> AbstractDomain for SimpleLattice<T> {
  fn bottom() -> SimpleLattice<T> {
    return SimpleLattice::<T>::Bottom;
  }

  fn top() -> SimpleLattice<T> {
    return SimpleLattice::<T>::Top;
  }

  fn is_bottom(&self) -> bool {
    return match self {
      SimpleLattice::Bottom => true,
      _ => false,
    }
  }

  fn is_top(&self) -> bool {
    return match self {
      SimpleLattice::Top => true,
      _ => false,
    }
  }

  fn leq(&self, rhs: &Self) -> bool {
    use SimpleLattice::*;
    return match self {
      Top => match rhs {
        Top => true,
        _ => false,
      },
      Value(x) => match rhs {
        Top => true,
        Value(y) => x == y,
        Bottom => false,
      },
      Bottom => true,
    }
  }

  fn join(self, rhs: Self) -> Self {
    use SimpleLattice::*;
    return match self {
      Top => Top,
      Value(ref x) => match rhs {
        Top => Top,
        Value(ref y) => if x == y { self } else { Top },
        Bottom => rhs,
      },
      Bottom => rhs,
    }
  }

  fn meet(self, rhs: Self) -> Self {
    use SimpleLattice::*;
    return match self {
      Top => rhs,
      Value(ref x) => match rhs {
        Top => self,
        Value(ref y) => if x == y { self } else { Bottom },
        Bottom => Bottom,
      },
      Bottom => Bottom,
    }
  }

  fn widen(self, rhs: Self) -> Self {
    return self.join(rhs);
  }

  fn narrow(self, rhs: Self) -> Self {
    return self.meet(rhs);
  }
}
