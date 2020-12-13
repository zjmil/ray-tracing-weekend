
#include "ray.h"

Point3 Ray_at(const Ray *ray, double t)
{
    // res = origin + t * direction
    return Vec3_add(Vec3_mul(&ray->direction, t), &ray->origin);
}
