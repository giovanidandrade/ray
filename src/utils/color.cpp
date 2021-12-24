#include "color.h"
#include <cmath>
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
Color::operator+(float s) const
{
  return Color(s + r, s + g, s + b);
}

Color
operator+(const float s, const Color& color)
{
  return Color(s + color.r, s + color.g, s + color.b);
}

Color
Color::operator*(const Color& c) const
{
  return Color(c.r * r, c.g * g, c.b * b);
}

Color
Color::operator+(const Color& c) const
{
  return Color(c.r + r, c.g + g, c.b + b);
}

void
Color::dump() const
{
  Color color = this->clamp().gamma2();

  printf("%d %d %d\n",
         static_cast<int>(256 * color.r),
         static_cast<int>(256 * color.g),
         static_cast<int>(256 * color.b));
}

Color
Color::clamp() const
{
  float newR = r < 0 ? 0 : r > 1 ? 1 : r;
  float newG = g < 0 ? 0 : g > 1 ? 1 : g;
  float newB = b < 0 ? 0 : b > 1 ? 1 : b;

  return Color(newR, newG, newB);
}

Color
Color::gamma2() const
{
  return Color(sqrt(r), sqrt(g), sqrt(b));
}