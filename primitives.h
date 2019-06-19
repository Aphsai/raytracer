#pragma once
#include "collideable.h"

class CollideableList : public Collideable {
	public:
		CollideableList() {}
		CollideableList(Collideable **l, int n) { list = l; list_size = n; }
		virtual bool hit(const ray& r, float t_min, float t_max, HitRecord& rec) const;
		Collideable **list;
		int list_size;
};

bool CollideableList::hit(const ray& r, float t_min, float t_max, HitRecord& rec) const {
	HitRecord temp_rec;
	bool hit_anything = false;
	double closest_so_far = t_max;
	for (int i = 0; i < list_size; i++) {
		if (list[i]->hit(r, t_min, closest_so_far, temp_rec)) {
			hit_anything = true;
			closest_so_far = temp_rec.t;
			rec = temp_rec;
		}
	}
	return hit_anything;
}

class Sphere: public Collideable {
	public:
		Sphere() {}
		Sphere(vec3 c, float r) { center = c; radius = r; }
		virtual bool hit(const ray& r, float t_min, float t_max, HitRecord& rec) const;
		vec3 center;
		float radius;
};

bool Sphere::hit(const ray& r, float t_min, float t_max, HitRecord& rec) const {
	vec3 oc = r.origin() - center;
	float a = dot(r.direction(), r.direction());
	float b = 2.0 * dot (oc, r.direction());
	float c = dot (oc, oc) - radius * radius;

	float discriminant = (b * b - 4 * a * c);
	if (discriminant > 0) {
		float discriminant_root = sqrt(discriminant);
		float time_till_hit = (-b - discriminant_root) / (2 * a);
		if (time_till_hit < t_max && time_till_hit > t_min) {
			rec.t = time_till_hit;
			rec.p = r.point_at_parameter(time_till_hit);
			rec.normal = (rec.p - center) / radius;
			return true;
		}
		time_till_hit = (-b + discriminant_root) / (2 * a);
		if (time_till_hit > t_max && time_till_hit < t_min) {
			rec.t = time_till_hit;
			rec.p = r.point_at_parameter(time_till_hit);
			rec.normal = (rec.p - center) / radius;
			return true;
		}
	}
	return false;
}
