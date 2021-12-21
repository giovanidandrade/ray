#include "vec.h"
#include <cmath>

Vec::Vec()
{
  this->x = 0;
  this->y = 0;
  this->z = 0;
}

Vec::Vec(float x, float y, float z)
{
  this->x = x;
  this->y = y;
  this->z = z;
}

Vec
Vec::operator-() const
{
  return Vec(-x, -y, -z);
}

Vec&
Vec::operator+=(const Vec& v)
{
  x += v.x;
  y += v.y;
  z += v.z;
  return *this;
}

Vec&
Vec::operator*=(float t)
{
  x *= t;
  y *= t;
  z *= t;
  return *this;
}

Vec&
Vec::operator/=(float t)
{
  x /= t;
  y /= t;
  z /= t;
  return *this;
}

Vec
Vec::operator+(const Vec& v) const
{
  return Vec(x + v.x, y + v.y, z + v.z);
}

Vec
Vec::operator-(const Vec& v) const
{
  return Vec(x - v.x, y - v.y, z - v.z);
}

Vec
Vec::operator*(float t) const
{
  return Vec(x * t, y * t, z * t);
}

Vec
Vec::operator/(float t) const
{
  float s = 1.0 / t;
  return (*this) * s;
}

Vec
operator*(float t, const Vec& v)
{
  return Vec(v.x * t, v.y * t, v.z * t);
}

Vec
operator/(float t, const Vec& v)
{
  float s = 1.0 / t;
  return v * s;
}

float
Vec::len_squared() const
{
  return x * x + y * y + z * z;
}

float
Vec::len() const
{
  return sqrt(len_squared());
}

Vec
Vec::normalize() const
{
  return (*this) / len();
}

float
Vec::dot(const Vec& v) const
{
  return x * v.x + y * v.y + z * v.z;
}

Vec
Vec::cross(const Vec& v) const
{
  return Vec(y * v.z - z * v.y, z * v.x - x * v.z, x * v.y - y * v.x);
}