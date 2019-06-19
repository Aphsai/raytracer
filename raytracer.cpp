#include <iostream>
#include <limits>
#include <fstream>
#include <stdlib.h>
#include <time.h>
#include "vec3.h"
#include "ray.h"
#include "primitives.h"
#include "camera.h"

float get_random_float() {
	return (float)(rand()) / RAND_MAX;
}

vec3 random_in_unit_sphere() {
	vec3 p;
	//first time using a do-while very exciting
	do {
		p = 2.0 * vec3 (get_random_float(), get_random_float(), get_random_float()) - vec3(1, 1, 1);
	} while (p.squared_length() >= 1.0);
	return p;
}

vec3 color(const ray& r, Collideable *world) {
	HitRecord rec;
	if (world->hit(r, 0.001, std::numeric_limits<float>::max(), rec)) {
		vec3 target = rec.p + rec.normal + random_in_unit_sphere();
		return 0.5 * color( ray(rec.p, target - rec.p), world);
	}

	vec3 unit_direction = unit_vector(r.direction());
	float t = 0.5 * (unit_direction.y() + 1.0);
	return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}

int main() {
	std::ofstream file;
	const int ROWS = 100;
	const int COLUMNS = 200;
	const int SAMPLES = 100;

	Collideable* list[2];
	list[0] = new Sphere(vec3(0, 0, -1), 0.5);
	list[1] = new Sphere(vec3(0, -100.5, -1), 100);
	Collideable* world = new CollideableList(list, 2);

	Camera camera;
	srand(time(NULL));
	
	file.open("ray_test.ppm");
	file << "P3\n" << COLUMNS << " " << ROWS << "\n255\n";
	for (int row = ROWS - 1; row >= 0; row--) {
		for (int col = 0; col < COLUMNS; col++) {
			vec3 averaged_color (0, 0, 0);
			for (int s = 0; s < SAMPLES; s++) {
				float u = float(col + get_random_float()) / float(COLUMNS);
				float v = float(row + get_random_float()) / float(ROWS);
				ray r = camera.get_ray(u, v);
				averaged_color += color(r, world);
			}
			averaged_color /= float(SAMPLES);
			averaged_color = vec3( sqrt(averaged_color.x()), sqrt(averaged_color.y()) ,sqrt(averaged_color.z()));
			int ir = int(averaged_color.x() * 255.99);
			int ig = int(averaged_color.y() * 255.99);
			int ib = int(averaged_color.z() * 255.99);
			file << ir << " " << ig << " " << ib << "\n";
		}
	}
	file.close();
}
