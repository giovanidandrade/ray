#include "sphere.h"
#include <cmath>

Sphere::Sphere(const Point& center,
               float radius,
               std::shared_ptr<Material> material)
{
  this->center = center;
  this->radius = radius;
  this->material = material;
}

std::optional<Observation>
Sphere::hit(const Ray& ray, float tMin, float tMax) const
{
  Vec oc = ray.origin - center;

  float a = ray.direction.lenSquared();
  float halfB = ray.direction.dot(oc);
  float c = oc.lenSquared() - radius * radius;

  float delta = halfB * halfB - a * c;
  if (delta < 0) {
    return {};
  }

  float sqrtD = sqrt(delta);
  float root = (-halfB - sqrtD) / a;

  if (root < tMin) {
    root = (-halfB + sqrtD) / a;
    if (root < tMin || tMax < root) {
      return {};
    }

  } else if (root > tMax) {
    // This is the most negative root, if it's already too big
    // then the other root will be bigger too
    return {};
  }

  Observation obs;
  obs.t = root;
  obs.point = ray.at(root);
  obs.material = material;

  Vec outwardNormal = (obs.point - center) / radius;
  obs.setFace(ray, outwardNormal);

  return obs;
}
