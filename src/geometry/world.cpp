#include "world.h"

World::World()
{
  this->objects = std::vector<Rc<Observable>>();
}

/* Moves objects into world */
World::World(WorldList& objects)
{
  this->objects = std::move(objects);
}

void
World::addObject(Rc<Observable> object)
{
  objects.push_back(object);
}

std::optional<Observation>
World::hit(const Ray& ray, float tMin, float tMax) const
{
  bool hitAnything = false;
  Observation obs;
  float closestSoFar = tMax;

  for (const auto& object : objects) {
    if (auto obsTemp = object->hit(ray, tMin, closestSoFar)) {
      hitAnything = true;
      closestSoFar = obsTemp->t;
      obs = *obsTemp;
    }
  }

  if (hitAnything) {
    return obs;
  } else {
    return {};
  }
}