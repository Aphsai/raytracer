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
