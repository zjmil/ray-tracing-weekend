
#include <stdio.h>
#include <math.h>

#include "vec3.h"
#include "ray.h"
#include "color.h"


void write_color(FILE *stream, Color *color)
{
    int r = (int)(255.999 * color->x);
    int g = (int)(255.999 * color->y);
    int b = (int)(255.999 * color->z);

    fprintf(stream, "%d %d %d\n", r, g, b);
}

double hit_sphere(const Point3 *center, double radius, const Ray *ray) {
    Vec3 oc = Vec3_sub(&ray->origin, center);

    double a = Vec3_mag_squared(&ray->direction);
    double half_b = Vec3_dot(&oc, &ray->direction);
    double c = Vec3_mag_squared(&oc) - radius * radius;
    double discriminant = half_b * half_b - a * c;

    if (discriminant < 0) {
        return -1.0;
    } else {
        return (-half_b - sqrt(discriminant)) / a;
    }
}

Color *ray_color(const Ray *r, Color *color)
{
    Point3 center = {0, 0, -1};

    double t = hit_sphere(&center, 0.5, r);
    if (t > 0.0) {
        Point3 at = Ray_at(r, t);
        Vec3 N = Vec3_normalize(Vec3_sub(&at, t), &center);
        
        return Vec3_mul(, 0.5, color);
    } else {
        Vec3 unit_dir;
        Vec3_normalize(&r->direction, &unit_dir);
        t = 0.5 * (unit_dir.y +  1.0);

        Color p1 = {1.0, 1.0, 1.0};
        Color p2 = {0.5, 0.7, 1.0};
        return Vec3_add(Vec3_mul(&p1, 1.0 - t, &p1), Vec3_mul(&p2, t, &p2), color);
    }
}

int main(void)
{
    // Image
    const double aspect_ratio = 16.0 / 9.0;
    const int image_width = 400;
    const int image_height = (int)(image_width / aspect_ratio);

    // Camera
    double viewport_height = 2.0;
    double viewport_width = aspect_ratio * viewport_height;
    double focal_length = 1.0;

    Point3 origin = {0, 0, 0};
    Vec3 horizontal = {viewport_width, 0, 0};
    Vec3 vertical = {0, viewport_height, 0};

    Vec3 half_horz = {viewport_width / 2.0, 0, 0};
    Vec3 half_vert = {0, viewport_height / 2.0, 0};

    Vec3 focal_vec = {0, 0, focal_length};
    Vec3 lower_left = origin;
    Vec3_sub(&lower_left, &half_horz, &lower_left);
    Vec3_sub(&lower_left, &half_vert, &lower_left);
    Vec3_sub(&lower_left, &focal_vec, &lower_left);

    // Render
    printf("P3\n%d %d\n255\n", image_width, image_height);

    for (int j = image_height-1; j >= 0; --j) {
        fprintf(stderr, "\rScanlines remaining: %d", j);
        fflush(stderr);
        for (int i = 0; i < image_width; ++i) {
            double u = (double)i / (image_width-1);
            double v = (double)j / (image_height-1);

            Ray ray = {.origin = origin, .direction = lower_left};
            Vec3 tmp1;

            Vec3_mul(&horizontal, u, &tmp1);
            Vec3_add(&ray.direction, &tmp1, &ray.direction);
            Vec3 tmp2;
            Vec3_mul(&vertical, v, &tmp2);
            Vec3_add(&ray.direction, &tmp2, &ray.direction);

            Vec3_sub(&ray.direction, &origin, &ray.direction);

            Color color;
            ray_color(&ray, &color);
            write_color(stdout, &color);
        }
    }

    fprintf(stderr, "\nDone.\n");

    return 0;
}
