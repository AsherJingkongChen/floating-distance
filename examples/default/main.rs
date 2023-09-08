mod cosine_similarity;
mod euclidean_distance;
mod metric_polymorphism;

use cosine_similarity::*;
use euclidean_distance::*;
use metric_polymorphism::*;

fn main() {
  cosine_similarity();
  euclidean_distance();
  metric_polymorphism();
}
