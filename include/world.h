#ifndef WORLD_H
#define WORLD_H

#include "observable.h"
#include <memory>
#include <vector>

template<class T>
using Rc = std::shared_ptr<T>;

using WorldList = std::vector<Rc<Observable>>;

class World : public Observable
{
public:
  World();
  World(WorldList& objects);

  WorldList objects;

  void addObject(Rc<Observable> object);

  virtual std::optional<Observation> hit(const Ray& ray,
                                         float tMin,
                                         float tMax) const override;

  virtual std::optional<AxisBox> boundingBox() const override;
};

#endif