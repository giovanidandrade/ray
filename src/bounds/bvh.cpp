#include "bvh.h"
#include "rand.h"
#include <algorithm>
#include <cstdio>

bool
compareBox(const Rc<Observable> a, const Rc<Observable> b, int axis)
{
  AxisBox boxA, boxB;

  if (auto box = a->boundingBox()) {
    boxA = *box;
  } else {
    fprintf(stderr, "No bounding box in BVH constructor.\n");
  }

  if (auto box = b->boundingBox()) {
    boxB = *box;
  } else {
    fprintf(stderr, "No bounding box in BVH constructor.\n");
  }

  return boxA.min[axis] < boxB.min[axis];
}

BVH::BVH(const WorldList& list, int start, int end)
{
  auto objects = list;

  int axis = randomInt(0, 2);
  auto comparator = [axis](Rc<Observable> a, Rc<Observable> b) {
    return compareBox(a, b, axis);
  };

  int objectSpan = end - start;

  if (objectSpan == 1) {
    left = right = objects[start];
  } else if (objectSpan == 2) {
    if (comparator(objects[start], objects[start + 1])) {
      left = objects[start];
      right = objects[start + 1];
    } else {
      left = objects[start + 1];
      right = objects[start];
    }
  } else {
    std::sort(objects.begin() + start, objects.begin() + end, comparator);

    int mid = start + objectSpan / 2;

    left = std::make_shared<BVH>(objects, start, mid);
    right = std::make_shared<BVH>(objects, mid, end);
  }

  AxisBox boxLeft, boxRight;
  if (auto box = left->boundingBox()) {
    boxLeft = *box;
  } else {
    fprintf(stderr, "No bounding box in BVH constructor.\n");
  }

  if (auto box = right->boundingBox()) {
    boxRight = *box;
  } else {
    fprintf(stderr, "No bounding box in BVH constructor.\n");
  }

  box = surroundingBox(boxLeft, boxRight);
}

std::optional<Observation>
BVH::hit(const Ray& ray, float tMin, float tMax) const
{
  if (box.hit(ray, tMin, tMax)) {
    Observation observation;
    bool hasHit = false;
    float tBig = tMax;

    if (auto obs = left->hit(ray, tMin, tMax)) {
      observation = *obs;
      tBig = obs->t;
      hasHit = true;
    }

    if (auto obs = right->hit(ray, tMin, tBig)) {
      observation = *obs;
      hasHit = true;
    }

    if (hasHit) {
      return observation;
    } else {
      return {};
    }
  }

  return {};
}

std::optional<AxisBox>
BVH::boundingBox() const
{
  return box;
}