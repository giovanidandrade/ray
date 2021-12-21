#ifndef PPM_H
#define PPM_H

#include "color.h"
#include <vector>

class PPM
{
public:
  PPM(int width, int height);
  void dump() const;

  void setColor(int x, int y, Color color);
  Color getColor(int x, int y) const;

private:
  int width;
  int height;
  std::vector<Color> buffer;

  int getIndex(int x, int y) const;
};

#endif