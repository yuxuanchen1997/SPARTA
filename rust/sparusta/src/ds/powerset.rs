/*
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

use super::abstract_domain::AbstractDomain;
use std::collections::HashSet;
use std::hash::Hash;

// Avoid using HashSet<T> directly? Use another type arguement that
// requires it to have a "Set" trait implemented.
#[derive(PartialEq, Eq, Clone)]
pub enum PowersetLattice<T: Eq + Hash + Clone> {
  Top,
  Value(HashSet<T>),
  Bottom,
}

// trait SetOps<T> {
//   fn iter(&self) -> Iterator;
//   fn contains<Q: ?Sized>(&self, value: &Q) -> bool;
// }

impl<T: Eq + Hash + Clone> AbstractDomain for PowersetLattice<T> {
  fn bottom() -> PowersetLattice<T> {
    return PowersetLattice::<T>::Bottom;
  }

  fn top() -> PowersetLattice<T> {
    return PowersetLattice::<T>::Top;
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
        Value(ref t) => Value(s.union(&t).cloned().collect()),
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
        Value(ref t) => Value(s.intersection(&t).cloned().collect()),
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
