#pragma once
#include "ray.h"

class Camera {
	public:
		Camera() { 
			origin = vec3(0.0, 0.0, 0.0);
			vertical = vec3(0.0, 2.0, 0.0);
			horizontal = vec3(4.0, 0.0, 0.0);
			ll_corner = vec3(-2.0, -1.0, -1.0);
		}
		ray get_ray(float u, float v) { return ray(origin, ll_corner + u * horizontal + v * vertical - origin); }
			vec3 origin;
			vec3 vertical;
			vec3 horizontal;
			vec3 ll_corner;
};
