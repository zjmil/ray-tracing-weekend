#ifndef RAY_H
#define RAY_H

#include "vec3.h"

typedef Vec3 Point3;

typedef struct {
    Point3 origin;
    Vec3 direction;
} Ray;


Point3 Ray_at(const Ray *ray, double t);

#endif /* RAY_H */
