/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use super::abstract_domain::AbstractDomain;
use std::collections::HashSet;
use std::hash::Hash;

pub trait SetOps {
  fn is_subset(&self, other: &Self) -> bool;
  fn intersection(&self, other: &Self) -> Self;
  fn union(&self, other: &Self) -> Self;
}

impl<T: Eq + Hash + Clone> SetOps for HashSet<T> {
  fn is_subset(&self, other: &Self) -> bool {
    return self.is_subset(other);
  }

  fn intersection(&self, other: &Self) -> Self {
    return self.intersection(&other).cloned().collect();
  }

  fn union(&self, other: &Self) -> Self {
    return self.union(&other).cloned().collect();
  }
}

#[derive(PartialEq, Eq, Clone)]
pub enum PowersetLattice<S: SetOps> {
  Top,
  Value(S),
  Bottom,
}

impl<S: SetOps> AbstractDomain for PowersetLattice<S> {
  fn bottom() -> PowersetLattice<S> {
    return PowersetLattice::<S>::Bottom;
  }

  fn top() -> PowersetLattice<S> {
    return PowersetLattice::<S>::Top;
  }

  fn is_bottom(&self) -> bool {
    return match self {
      PowersetLattice::Bottom => true,
      _ => false,
    };
  }

  fn is_top(&self) -> bool {
    return match self {
      PowersetLattice::Top => true,
      _ => false,
    };
  }

  fn leq(&self, rhs: &Self) -> bool {
    use PowersetLattice::*;
    return match self {
      Top => rhs.is_top(),
      Value(ref s) => match rhs {
        Top => true,
        Value(ref t) => s.is_subset(&t),
        Bottom => false,
      },
      Bottom => true,
    };
  }

  fn join(self, rhs: Self) -> Self {
    use PowersetLattice::*;
    return match self {
      Top => self,
      Value(ref s) => match rhs {
        Top => rhs,
        // Is this ineffecient? Can we just ref mut s itself?
        Value(ref t) => Value(s.union(t)),
        Bottom => self,
      },
      Bottom => rhs,
    };
  }

  fn meet(self, rhs: Self) -> Self {
    use PowersetLattice::*;
    return match self {
      Top => rhs,
      Value(ref s) => match rhs {
        Top => self,
        Value(ref t) => Value(s.intersection(t)),
        Bottom => rhs,
      },
      Bottom => self,
    };
  }

  fn widen(self, rhs: Self) -> Self {
    return self.join(rhs);
  }

  fn narrow(self, rhs: Self) -> Self {
    return self.meet(rhs);
  }
}
