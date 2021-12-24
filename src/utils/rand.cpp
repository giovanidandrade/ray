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
randomFloat(float min, float max)
{
  return randomFloat() * (max - min) + min;
}

float
jitter(float f)
{
  return f + randomFloat();
}

Vec
randomVec(float min, float max)
{
  return Vec(
    randomFloat(min, max), randomFloat(min, max), randomFloat(min, max));
}

Vec
randomUnitVector()
{
  return randomInUnitSphere().normalize();
}

Vec
randomInUnitSphere()
{
  while (true) {
    Vec v = randomVec(-1, 1);
    if (v.lenSquared() < 1) {
      return v;
    }
  }
}

Vec
randomInUnitDisk()
{
  while (true) {
    Vec v = randomVec(-1, 1);
    v.z = 0;

    if (v.lenSquared() < 1) {
      return v;
    }
  }
}