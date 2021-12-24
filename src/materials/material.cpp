#include "material.h"
#include <cmath>

Vec
refract(const Vec& ray, const Vec& normal, float ratio)
{
  float cosTheta = fmin(-ray.dot(normal), 1.0);

  Vec refract_perp = ratio * (ray + cosTheta * normal);
  Vec refract_para = -sqrt(fabs(1.0 - refract_perp.lenSquared())) * normal;

  return refract_perp + refract_para;
}

Vec
reflect(const Vec& ray, const Vec& normal)
{
  return ray - 2 * ray.dot(normal) * normal;
}