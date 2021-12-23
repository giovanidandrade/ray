#ifndef LAMBERTIAN_H
#define LAMBERTIAN_H

#include "material.h"

class Lambertian : public Material
{
public:
  Lambertian(const Color& albedo);

  virtual std::optional<Pair> scatter(const Ray& ray,
                                      const Observation& obs) const override;

private:
  Color albedo;
};

#endif