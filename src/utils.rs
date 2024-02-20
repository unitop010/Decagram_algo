pub fn polygonal_rectangle(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      1f64,
      1f64,
      (number) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_triangle(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      1f64,
      1f64,
      (number * 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_square(number: u64) -> Option<u64> {
  if is_perfect_square(number as u128) {
      let value = (number as f64).sqrt() as u64;
      return Some(value);
  }
  None
}

pub fn polygonal_pentagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      3f64,
      -1f64,
      (number * 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_hexagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      2f64,
      -1f64,
      number as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_heptagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      5f64,
      -3f64,
      (number * 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_octagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      3f64,
      -2f64,
      number as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_nonagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      7f64,
      -5f64,
      (number * 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_decagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      4f64,
      -3f64,
      number as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_star(number: u64) -> Option<u64> {
  let num: f64 = if number == 1 {
      0.0
  } else {
      (number - 1) as f64 * (-1f64)
  };

  match Root(roots::find_roots_quadratic(6f64, -6f64, num))
      .to_string()
      .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_triangle(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      3f64,
      -3f64,
      ((number * 2) - 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_square(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      2f64,
      -2f64,
      (number - 1) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_pentagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      5f64,
      -5f64,
      ((number * 2) - 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_hexagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      3f64,
      -3f64,
      (number - 1) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_heptagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      7f64,
      -7f64,
      ((number * 2) - 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_octagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      4f64,
      -4f64,
      (number - 1) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_nonagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      9f64,
      -9f64,
      ((number * 2) - 2) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}

pub fn polygonal_centered_decagon(number: u64) -> Option<u64> {
  match Root(roots::find_roots_quadratic(
      5f64,
      -5f64,
      (number - 1) as f64 * (-1f64),
  ))
  .to_string()
  .parse::<u64>()
  {
      Ok(n) => Some(n),
      Err(_) => None,
  }
}