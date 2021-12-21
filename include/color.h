#ifndef COLOR_H
#define COLOR_H

class Color
{
public:
  Color(float r, float g, float b);
  Color();

  Color operator*(const float s) const;

  float r, g, b;
};

Color
operator*(const float s, const Color& color);

#endif