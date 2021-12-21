#include "color.h"
#include <cstdio>

Color::Color(float r, float g, float b)
{
  this->r = r;
  this->g = g;
  this->b = b;
}

Color::Color()
{
  this->r = 0;
  this->g = 0;
  this->b = 0;
}

Color
Color::operator*(float s) const
{
  return Color(s * r, s * g, s * b);
}

Color
operator*(const float s, const Color& color)
{
  return Color(s * color.r, s * color.g, s * color.b);
}

Color
Color::operator+(const Color& c) const
{
  return Color(r + c.r, g + c.g, b + c.b);
}

void
Color::dump() const
{
  printf("%d %d %d\n",
         static_cast<int>(255.999 * r),
         static_cast<int>(255.999 * g),
         static_cast<int>(255.999 * b));
}