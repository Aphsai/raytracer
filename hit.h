#include "vec3.h"

class Hit {
	Hit(const vec3 p, const vec3 n){ point = p; normal = n; }
	vec3 point;
	vec3 normal;
};
