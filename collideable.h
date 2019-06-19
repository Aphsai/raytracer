#pragma once
#include "ray.h"

class Material;

struct HitRecord {
	float t;
	vec3 p;
	vec3 normal;
	Material *mat_ptr;
};

class Collideable {
	public:
		virtual bool hit(const ray& r, float t_min, float t_max, HitRecord& rec) const = 0;
};

