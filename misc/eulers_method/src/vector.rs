use itertools::Itertools; // 0.8.0
use std::error::Error;
use std::fmt;
use std::ops;

/// Struct representing a mathematical vector
#[derive(Clone)]
pub struct Vector {
  dimension: usize,
  components: Vec<f64>,
}

#[derive(Debug)]
pub struct VectorCreationError(String);

impl fmt::Display for VectorCreationError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for VectorCreationError {}

impl Vector {
  pub fn dimension(&self) -> usize {
    self.dimension
  }

  pub fn components(&self) -> &Vec<f64> {
    &self.components
  }

  pub fn new(dimension: usize, components: Vec<f64>) -> Result<Vector, VectorCreationError> {
    if components.len() != dimension {
      Err(VectorCreationError(String::from(
        "The length of the components vector must matched the passed dimension!",
      )))
    } else {
      Ok(Vector {
        dimension,
        components,
      })
    }
  }

  pub fn scale(vector: Vector, factor: f64) -> Vector {
    let scaled_components: Vec<f64> = vector
      .components
      .iter()
      .map(|component| component * factor)
      .collect();

    Vector {
      dimension: vector.dimension,
      components: scaled_components,
    }
  }
}

impl fmt::Display for Vector {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let formatted_components: String = self
      .components
      .iter()
      .map(|component| format!("{:.8}", component))
      .intersperse(", ".to_string())
      .collect();

    write!(f, "[{}]", formatted_components)
  }
}

#[derive(Debug)]
pub struct VectorAdditionError(String);

impl fmt::Display for VectorAdditionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl Error for VectorAdditionError {}

impl ops::Add for Vector {
  type Output = Result<Vector, VectorAdditionError>;

  fn add(self, rhs: Self) -> Result<Vector, VectorAdditionError> {
    if self.components.len() != rhs.components.len() {
      Err(VectorAdditionError(String::from(
        "Both vectors must have the same number of components!",
      )))
    } else {
      let added_components = self
        .components
        .iter()
        .zip(rhs.components.iter())
        .map(|(lhs, rhs)| lhs + rhs)
        .collect();

      Ok(Vector {
        dimension: self.components.len(),
        components: added_components,
      })
    }
  }
}
