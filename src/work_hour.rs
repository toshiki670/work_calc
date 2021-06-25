use std::{fmt, cmp, ops};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct WorkHour {
  value: i32,
  reminder: f32,
}

impl WorkHour {
  const QUARTER: f32 = 0.25;
  pub fn new(hour: f32) -> Self {
    let value: i32 = (hour / WorkHour::QUARTER) as i32;
    let reminder: f32 = hour % WorkHour::QUARTER;
    Self { value: value, reminder: reminder }
  }

  pub fn raw(&self) -> f32 {
    self.value as f32 * WorkHour::QUARTER + self.reminder
  }

  pub fn hour(&self) -> f32 {
    self.value as f32 * WorkHour::QUARTER
  }

  pub fn reminder(&self) -> f32 {
    self.reminder
  }
}

impl fmt::Display for WorkHour {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
     write!(f, "{:.2}", self.hour())
  }
}

impl cmp::PartialEq<f32> for WorkHour {
  fn eq(&self, other: &f32) -> bool {
    let other = WorkHour::new(*other);
    self.value == other.value
  }
}

impl ops::Add for WorkHour {
  type Output = Self;
  fn add(self, other: Self) -> Self::Output {
    Self::new(self.raw() + other.raw())
  }
}

impl ops::Add<f32> for WorkHour {
  type Output = Self;
  fn add(self, other: f32) -> Self::Output {
    Self::new(self.raw() + other)
  }
}

impl ops::Sub for WorkHour {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    Self::new(self.raw() - other.raw())
  }
}

impl ops::Sub<f32> for WorkHour {
  type Output = Self;
  fn sub(self, other: f32) -> Self::Output {
    Self::new(self.raw() - other)
  }
}

impl ops::Mul for WorkHour {
  type Output = Self;
  fn mul(self, other: Self) -> Self::Output {
    Self::new(self.raw() * other.raw())
  }
}

impl ops::Mul<f32> for WorkHour {
  type Output = Self;
  fn mul(self, other: f32) -> Self::Output {
    Self::new(self.raw() * other)
  }
}

impl ops::Div for WorkHour {
  type Output = Self;
  fn div(self, other: Self) -> Self::Output {
    Self::new(self.raw() / other.raw())
  }
}

impl ops::Div<f32> for WorkHour {
  type Output = Self;
  fn div(self, other: f32) -> Self::Output {
    Self::new(self.raw() / other)
  }
}

impl ops::Rem for WorkHour {
  type Output = Self;
  fn rem(self, other: Self) -> Self::Output {
    Self::new(self.raw() % other.raw())
  }
}

impl ops::Rem<f32> for WorkHour {
  type Output = Self;
  fn rem(self, other: f32) -> Self::Output {
    Self::new(self.raw() % other)
  }
}


#[cfg(test)]
mod tests {
  #[test]
  fn be_valid_value() {
    let wh = super::WorkHour::new(170.50);
    assert_eq!(wh, super::WorkHour::new(170.50));
    assert_eq!(wh, 170.50);
    assert_eq!(wh.to_string(), "170.50");
  }

  #[test]
  fn be_valid_add() {
    let test1 = super::WorkHour::new(145.25) + super::WorkHour::new(0.25);
    assert_eq!(test1, 145.5);
    let test2 = super::WorkHour::new(145.25) + 0.25;
    assert_eq!(test2, 145.5);
  }

  #[test]
  fn be_valid_minus() {
    let test1 = super::WorkHour::new(145.25) - super::WorkHour::new(1.25);
    assert_eq!(test1, 144.0);
    let test2 = super::WorkHour::new(145.25) - 2.50;
    assert_eq!(test2, 142.75);
  }

  #[test]
  fn be_valid_times() {
    let test1 = super::WorkHour::new(145.25) * super::WorkHour::new(1.25);
    assert_eq!(test1, 181.5);
    assert_eq!(test1.reminder(), 0.0625);
    let test2 = super::WorkHour::new(145.25) * 2.50;
    assert_eq!(test2, 363.0);
    assert_eq!(test2.reminder(), 0.125);
    let test3 = super::WorkHour::new(145.25) * 0.30;
    assert_eq!(test3, 43.5);
    assert_eq!(test3.reminder(), 0.07500076);
  }

  #[test]
  fn be_valid_div() {
    let test1 = super::WorkHour::new(145.25) / super::WorkHour::new(1.25);
    assert_eq!(test1, 116.0);
    let test2 = super::WorkHour::new(145.25) / 2.50;
    assert_eq!(test2, 58.0);
    let test3 = super::WorkHour::new(145.25) / 0.30;
    assert_eq!(test3, 484.0);
  }

  #[test]
  fn be_valid_rem() {
    let test1 = super::WorkHour::new(145.25) % super::WorkHour::new(1.25);
    assert_eq!(test1, 116.0);
    let test2 = super::WorkHour::new(145.25) % 2.50;
    assert_eq!(test2, 58.0);
    let test3 = super::WorkHour::new(145.25) % 0.30;
    assert_eq!(test3, 484.0);
    let test4 = super::WorkHour::new(145.25) % 150.0;
    assert_eq!(test4, 484.0);
  }
}