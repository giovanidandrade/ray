#ifndef COLOR_H
#define COLOR_H

class Color
{
public:
  Color(float r, float g, float b);
  Color();

  Color operator*(const float s) const;
  Color operator+(const Color& c) const;

  float r, g, b;

  void dump() const;
};

Color
operator*(const float s, const Color& color);

#endif