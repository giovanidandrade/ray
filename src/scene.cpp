#include "scene.h"
#include "sphere.h"
#include "vec.h"
#include <limits>

using std::make_shared;

const float infinity = std::numeric_limits<float>::infinity();

World
makeWorld()
{
  WorldList objects = WorldList(2);

  objects[0] = make_shared<Sphere>(Point(0, 0, -1), 0.5);
  objects[1] = make_shared<Sphere>(Point(0, -100.5, -1), 100);

  return World(objects);
}

Color
getColor(const Ray& ray, const World& world)
{
  if (auto obs = world.hit(ray, 0, infinity)) {
    return 0.5 * (obs->normal.toColor() + Color(1, 1, 1));
  }

  Vec unit = ray.direction.normalize();

  float t = 0.5 * (unit.y + 1.0);
  return t * Color(0.5, 0.7, 1.0) + (1.0 - t) * Color(1, 1, 1);
}