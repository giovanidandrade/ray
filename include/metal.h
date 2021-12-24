#ifndef METAL_H
#define METAL_H

#include "material.h"

class Metal : public Material
{
public:
  Metal(const Color& sheen);
  Metal(const Color& sheen, float fuzz);

  virtual std::optional<Pair> scatter(const Ray& ray,
                                      const Observation& obs) const override;

private:
  Color sheen;
  float fuzz;
};

#endif