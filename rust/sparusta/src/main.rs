mod sparta;

fn main() {
  use sparta::*;
  type ConstDomain = SimpleLattice::<i64>;
  let top = ConstDomain::Top;
  let value = ConstDomain::Value(42);

  assert!(top.is_top());
  assert!(!value.is_top());
  assert!(value.leq(&top));
  assert!(ConstDomain::Bottom.leq(&top));
}
