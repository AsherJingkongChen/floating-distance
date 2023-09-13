mod cosine_similarity;
mod euclidean_distance;
mod manhattan_distance;

use cosine_similarity::*;
use euclidean_distance::*;
use manhattan_distance::*;

fn main() {
  cosine_similarity();
  euclidean_distance();
  manhattan_distance();
}
