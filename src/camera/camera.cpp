#include "camera.h"

Camera::Camera(float viewportHeight, float aspectRatio, float focalLength)
{
  float viewportWidth = aspectRatio * viewportHeight;

  this->origin = Point(0, 0, 0);
  this->horizontal = Vec(viewportWidth, 0, 0);
  this->vertical = Vec(0, viewportHeight, 0);
  this->lowerLeftCorner =
    origin - horizontal / 2.0 - vertical / 2.0 - Vec(0, 0, focalLength);
}

Ray
Camera::cast(float u, float v) const
{
  return Ray(origin, lowerLeftCorner + u * horizontal + v * vertical - origin);
}