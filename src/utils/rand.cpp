#include "rand.h"

#include <random>

float
randomFloat()
{
  // This should be thread safe, althought it does leak 2.5 KiB per thread.
  // This should be acceptable, but if it isn't, you'll need to roll your
  // own thread safe randomness.
  static thread_local std::mt19937_64 generator;
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