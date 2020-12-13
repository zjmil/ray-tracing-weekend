#include <math.h>

#include "vec3.h"

Vec3 Vec3_add(const Vec3 *v, const Vec3 *u)
{
    return (Vec3){v->x + u->x, v->y + u->y, v->z + u->z};
}

Vec3 Vec3_sub(const Vec3 *v, const Vec3 *u)
{
    return (Vec3){v->x - u->x, v->y - u->y, v->z - u->z};
}

Vec3 Vec3_mul(const Vec3 *v, double a)
{
    return (Vec3){v->x * a, v->y * a, v->z * a};
}

Vec3 Vec3_div(const Vec3 *v, double a)
{
    return (Vec3){v->x / a, v->y / a, v->z / a};
}

Vec3 Vec3_neg(const Vec3 *v)
{
    return (Vec3){-v->x, -v->y, -v->z};
}

double Vec3_mag(const Vec3 *v)
{
    return sqrt(Vec3_mag_squared(v));
}

double Vec3_mag_squared(const Vec3 *v)
{
    return Vec3_dot(v, v);
}

double Vec3_dot(const Vec3 *v, const Vec3 *u)
{
    return (v->x * u->x + v->y * u->y + v->z + u->z);
}

Vec3 Vec3_cross(const Vec3 *v, const Vec3 *u)
{
    return (Vec3){
        v->y * u->z - v->z * u->y,
        v->z * u->x - v->x * u->z,
        v->x * u->y - v->y * u->x
    };
}

Vec3 Vec3_normalize(const Vec3 *v)
{
    return Vec3_div(v, Vec3_mag(v));
}
