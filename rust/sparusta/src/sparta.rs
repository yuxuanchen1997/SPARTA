pub trait AbstractDomain {
  fn is_bottom(&self) -> bool;
  fn is_top(&self) -> bool;
  fn leq(&self, rhs: &Self) -> bool;
}

#[derive(PartialEq)]
#[allow(dead_code)]
pub enum SimpleLattice<T: PartialEq> {
  Top,
  Value(T),
  Bottom,
}

impl<T: PartialEq> AbstractDomain for SimpleLattice<T> {
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
}
