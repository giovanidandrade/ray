#ifndef DIELECTRIC_H
#define DIELECTRIC_H

#include "material.h"

class Dielectric : public Material
{
public:
  Dielectric(float indexRefraction);
  Dielectric(const Color& tint, float indexRefraction);

  virtual std::optional<Pair> scatter(const Ray& ray,
                                      const Observation& obs) const override;

private:
  Color tint;
  float indexRefraction;

  float reflectance(float cosine, float refraction) const;
};

#endif