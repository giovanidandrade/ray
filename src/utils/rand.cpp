#include "rand.h"

#include <random>

float
randomFloat()
{
  // This should be thread safe
  static thread_local std::mt19937 generator;
  std::uniform_real_distribution<float> distribution(0.0, 1.0);
  return distribution(generator);
}

float
jitter(float f)
{
  return f + randomFloat();
}