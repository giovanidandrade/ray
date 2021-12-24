#ifndef CAMERA_H
#define CAMERA_H

#include "ray.h"

class Camera
{
public:
  Camera(Point lookFrom,
         Point lookAt,
         Vec viewUp,
         float fov,
         float aspectRatio,
         float aperture,
         float focusDistance);
  Ray cast(float u, float v) const;

private:
  Point origin;
  Point lowerLeftCorner;
  Vec horizontal;
  Vec vertical;

  Vec lookVector, unitHorizontal, unitVertical;
  float lensRadius;
};

#endif