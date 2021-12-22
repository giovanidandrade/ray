#ifndef CAMERA_H
#define CAMERA_H

#include "ray.h"

class Camera
{
public:
  Camera(float viewportHeight, float aspectRatio, float focalLength);
  Ray cast(float u, float v) const;

private:
  Point origin;
  Point lowerLeftCorner;
  Vec horizontal;
  Vec vertical;
};

#endif