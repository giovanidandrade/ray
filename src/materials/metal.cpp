#include "metal.h"
#include "observable.h"
#include "rand.h"
#include <cstdio>

Metal::Metal(const Color& sheen)
{
  this->sheen = sheen;
  this->fuzz = 0;
}

Metal::Metal(const Color& sheen, float fuzz)
{
  this->sheen = sheen;
  if (fuzz < 0) {
    this->fuzz = 0;
  } else if (fuzz > 1) {
    this->fuzz = 1;
  } else {
    this->fuzz = fuzz;
  }
}

std::optional<Pair>
Metal::scatter(const Ray& ray, const Observation& obs) const
{
  Vec reflected = reflect(ray.direction.normalize(), obs.normal);

  Vec fuzziness = fuzz * randomInUnitSphere();
  Ray scattered = Ray(obs.point, reflected + fuzziness);
  Color attenuation = sheen;

  return std::make_pair(scattered, attenuation);
}