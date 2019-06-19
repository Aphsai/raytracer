#pragma once
#include "ray.h"

struct HitRecord {
	float t;
	vec3 p;
	vec3 normal;
};

class Collideable {
	public:
		virtual bool hit(const ray& r, float t_min, float t_max, HitRecord& rec) const = 0;
};

