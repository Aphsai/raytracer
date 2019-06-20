#pragma once
#include <stdlib.h>
#include <time.h>
#include "vec3.h"


float get_random_float() {
	return (float)(rand()) / RAND_MAX;
}

vec3 random_in_unit_sphere() {
	vec3 p;
	do {
		p = 2.0 * vec3 (get_random_float(), get_random_float(), get_random_float()) - vec3(1, 1, 1);
	} while (p.squared_length() >= 1.0);
	return p;
}

vec3 reflect(const vec3& v, const vec3& n) {
	return v - 2 * dot(v, n) * n;
}

bool refract (const vec3& v, const vec3& n, float ni_over_nt, vec3& refracted) {
	vec3 uv = unit_vector(v);
	float dt = dot(uv, n);
	float discriminant = 1.0 - ni_over_nt * ni_over_nt * (1 - dt * dt);
	if (discriminant > 0) {
		refracted = ni_over_nt * (uv - n * dt) - n * sqrt (discriminant);
		return true;
	}
	return false;
}
