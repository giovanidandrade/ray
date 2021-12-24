#ifndef BVH_H
#define BVH_H

#include "observable.h"
#include "world.h"

class BVH : public Observable
{
public:
  BVH();

  BVH(const World& world)
    : BVH(world.objects, 0, world.objects.size())
  {}

  BVH(const WorldList& objects, int start, int end);

  virtual std::optional<Observation> hit(const Ray& ray,
                                         float tMin,
                                         float tMax) const override;

  virtual std::optional<AxisBox> boundingBox() const override;

private:
  Rc<Observable> left;
  Rc<Observable> right;
  AxisBox box;
};

#endif