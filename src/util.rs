
pub fn ternary<T>(condition: bool, if_true: T, if_false: T) -> T {
  if condition {
    if_true
  } else {
    if_false
  }
}