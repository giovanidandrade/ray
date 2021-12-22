#ifndef VEC_H
#define VEC_H

#include "color.h"

class Vec
{
public:
  Vec();
  Vec(float x, float y, float z);

  float x, y, z;

  Vec operator-() const;
  Vec& operator+=(const Vec& v);
  Vec& operator*=(float t);
  Vec& operator/=(float t);

  Vec operator+(const Vec& v) const;
  Vec operator-(const Vec& v) const;
  Vec operator*(float t) const;
  Vec operator/(float t) const;

  float len() const;
  float lenSquared() const;

  Vec normalize() const;
  float dot(const Vec& v) const;
  Vec cross(const Vec& v) const;

  // Useful for testing / Normal materials
  Color toColor() const;
};

Vec
operator*(float t, const Vec& v);

Vec
operator/(float t, const Vec& v);

#endif