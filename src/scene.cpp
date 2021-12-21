#include "scene.h"
#include "vec.h"

Color
getColor(const Ray& ray)
{
  Vec unit = ray.direction.normalize();

  float t = 0.5 * (unit.y + 1.0);
  return t * Color(0.5, 0.7, 1.0) + (1.0 - t) * Color(1, 1, 1);
}