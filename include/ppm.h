#ifndef PPM_H
#define PPM_H

#include <vector>

class PPM
{
public:
  PPM(int width, int height);
  void dump() const;

  void setColor(int x, int y, float color);
  float getColor(int x, int y) const;

private:
  int width;
  int height;
  std::vector<float> buffer;

  int getIndex(int x, int y) const;
};

#endif