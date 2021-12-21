#include "ppm.h"
#include <cstdio>

PPM::PPM(int width, int height)
{
  this->width = width;
  this->height = height;
  this->buffer = std::vector<float>(width * height);
}

void
PPM::dump() const
{
  // Magic header for PPM
  printf("P3\n%d %d\n255\n", width, height);

  for (int y = 0; y < height; ++y) {
    for (int x = 0; x < width; ++x) {
      int color = static_cast<int>(255.9999 * getColor(x, y));
      printf("%d %d %d\n", color, color, color);
    }
  }
}

float
PPM::getColor(int x, int y) const
{
  int index = getIndex(x, y);
  return buffer[index];
}

void
PPM::setColor(int x, int y, float color)
{
  int index = getIndex(x, y);
  buffer[index] = color;
}

int
PPM::getIndex(int x, int y) const
{
  return x + width * y;
}