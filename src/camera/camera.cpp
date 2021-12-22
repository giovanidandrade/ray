#include "camera.h"

Camera::Camera(float viewport_height, float aspect_ratio, float focal_length)
{
  float viewport_width = aspect_ratio * viewport_height;

  this->origin = Point(0, 0, 0);
  this->horizontal = Vec(viewport_width, 0, 0);
  this->vertical = Vec(0, viewport_height, 0);
  this->lower_left_corner =
    origin - horizontal / 2.0 - vertical / 2.0 - Vec(0, 0, focal_length);
}

Ray
Camera::cast(float u, float v) const
{
  return Ray(origin,
             lower_left_corner + u * horizontal + v * vertical - origin);
}