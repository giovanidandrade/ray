#include "color.h"

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