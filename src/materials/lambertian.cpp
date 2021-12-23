#include "lambertian.h"
#include "observable.h"
#include "rand.h"

Lambertian::Lambertian(const Color& albedo)
{
  this->albedo = albedo;
}

std::optional<Pair>
Lambertian::scatter(const Ray& ray, const Observation& obs) const
{
  Vec scatterDirection = obs.normal + randomUnitVector();

  if (scatterDirection.isNearZero()) {
    scatterDirection = obs.normal;
  }

  Ray scatter = Ray(obs.point, scatterDirection);
  Color attenuation = albedo;

  return std::make_pair(scatter, attenuation);
}