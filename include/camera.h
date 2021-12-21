#ifndef CAMERA_H
#define CAMERA_H

#include "ray.h"

class Camera
{
public:
  Camera(float viewport_height, float aspect_ratio, float focal_length);
  Ray cast(float u, float v) const;

private:
  Point origin;
  Point lower_left_corner;
  Vec horizontal;
  Vec vertical;
};

#endif