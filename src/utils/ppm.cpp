#include "ppm.h"
#include <cstdio>

PPM::PPM(int width, int height)
{
  this->width = width;
  this->height = height;
  this->buffer = std::vector<Color>(width * height);
}

void
PPM::dump() const
{
  // Magic header for PPM
  printf("P3\n%d %d\n255\n", width, height);

  for (int y = 0; y < height; ++y) {
    for (int x = 0; x < width; ++x) {
      Color color = 255.9999 * getColor(x, y);
      printf("%d %d %d\n", (int)color.r, (int)color.g, (int)color.b);
    }
  }
}

Color
PPM::getColor(int x, int y) const
{
  int index = getIndex(x, y);
  return buffer[index];
}

void
PPM::setColor(int x, int y, Color color)
{
  int index = getIndex(x, y);
  buffer[index] = color;
}

int
PPM::getIndex(int x, int y) const
{
  return x + width * y;
}