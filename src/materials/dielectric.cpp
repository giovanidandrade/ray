#include "dielectric.h"
#include "observable.h"
#include "rand.h"
#include <cmath>

Dielectric::Dielectric(float indexRefraction)
{
  this->tint = Color(1, 1, 1);
  this->indexRefraction = indexRefraction;
}

Dielectric::Dielectric(const Color& tint, float indexRefraction)
{
  this->tint = tint;
  this->indexRefraction = indexRefraction;
}

float
Dielectric::reflectance(float cosine, float refraction) const
{
  float r0 = (1 - refraction) / (1 + refraction);
  r0 *= r0;

  return r0 + (1 - r0) * pow((1 - cosine), 5);
}

std::optional<Pair>
Dielectric::scatter(const Ray& ray, const Observation& obs) const
{
  Color attenuation = tint;

  float refractionRatio =
    obs.frontFacing ? 1.0 / indexRefraction : indexRefraction;

  Vec unitDirection = ray.direction.normalize();

  float cosTheta = fmin(-unitDirection.dot(obs.normal), 1.0);
  float sinTheta = sqrt(1 - cosTheta * cosTheta);

  bool cannotRefract = refractionRatio * sinTheta > 1.0;
  Vec direction;
  if (cannotRefract || reflectance(cosTheta, refractionRatio) > randomFloat()) {
    direction = reflect(unitDirection, obs.normal);
  } else {
    direction = refract(unitDirection, obs.normal, refractionRatio);
  }

  Ray scattered = Ray(obs.point, direction);
  return std::make_pair(scattered, attenuation);
}