#include <iostream>
#include <fstream>
#include "vec3.h"
#include "ray.h"
#include "primitives.h"
#include <limits>


//TODO: Move to primitives header file

vec3 color(const ray& r, Collideable *world) {
	HitRecord rec;
	if (world->hit(r, 0.0, std::numeric_limits<float>::max(), rec)) {
		return 0.5 * vec3(rec.normal.x() + 1, rec.normal.y() + 1, rec.normal.z() + 1);
	}

	vec3 unit_direction = unit_vector(r.direction());
	float t = 0.5 * (unit_direction.y() + 1.0);
	return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}

int main() {
	std::ofstream file;
	const int ROWS = 100;
	const int COLUMNS = 200;

	vec3 origin (0.0, 0.0, 0.0);
	vec3 vertical (0.0, 2.0, 0.0);
	vec3 horizontal (4.0, 0.0, 0.0);
	vec3 ll_corner (-2.0, -1.0, -1.0);

	Collideable* list[2];
	list[0] = new Sphere(vec3(0, 0, -1), 0.5);
	list[1] = new Sphere(vec3(0, -100.5, -1), 100);
	Collideable* world = new CollideableList(list, 2);
	
	file.open("ray_test.ppm");
	file << "P3\n" << COLUMNS << " " << ROWS << "\n255\n";
	for (int row = ROWS - 1; row >= 0; row--) {
		for (int col = 0; col < COLUMNS; col++) {
			ray r (origin, ll_corner + float(col) / float(COLUMNS) * horizontal + float(row) / float(ROWS) * vertical);
			vec3 v = color(r, world);
			int ir = int(v.x() * 255.99);
			int ig = int(v.y() * 255.99);
			int ib = int(v.z() * 255.99);
			file << ir << " " << ig << " " << ib << "\n";
		}
	}
	file.close();
}
