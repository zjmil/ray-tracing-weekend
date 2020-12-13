#ifndef VEC3_H
#define VEC3_H

typedef struct {
    double x, y, z;
} Vec3;


Vec3 Vec3_add(const Vec3 *a, const Vec3 *b);
Vec3 Vec3_sub(const Vec3 *a, const Vec3 *b);
Vec3 Vec3_mul(const Vec3 *a, double b);
Vec3 Vec3_div(const Vec3 *a, double b);

Vec3 Vec3_neg(const Vec3 *a);

double Vec3_mag(const Vec3 *v);
double Vec3_mag_squared(const Vec3 *v);
double Vec3_dot(const Vec3 *v, const Vec3 *u);
Vec3 Vec3_cross(const Vec3 *a, const Vec3 *b);
Vec3 Vec3_normalize(const Vec3 *a);

#endif
