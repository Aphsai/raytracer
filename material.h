#pragma once
#include "collideable.h"
#include "utility.h"


class Material {
	public:
		virtual bool scatter(const ray& r_in, const HitRecord& rec, vec3& attenuation, ray& scattered) const = 0;
		vec3 albedo;
};

class Lambertian : public Material {
	public:
		Lambertian(const vec3& a) { albedo = a; }
		virtual bool scatter(const ray& r_in, const HitRecord& rec, vec3& attenuation, ray& scattered) const {
			vec3 target = rec.p + rec.normal + random_in_unit_sphere();
			scattered = ray(rec.p, target - rec.p);
			attenuation = albedo;
			return true;
		}
};

class Metal : public Material {
	public:
		Metal(const vec3& a) { albedo = a; }
		Metal(const vec3& a, float f) { albedo = a; fuzziness = fmax(f, 1); }
		virtual bool scatter(const ray& r_in, const HitRecord& rec, vec3& attenuation, ray& scattered) const {
			vec3 reflected = reflect(r_in.direction(), rec.normal);
			scattered = ray(rec.p, reflected + fuzziness * random_in_unit_sphere());
			attenuation = albedo;
			return (dot(scattered.direction(), rec.normal) > 0);
		}
		float fuzziness = 0;

};
