#include <iostream>
#include <fstream>
#include "vec3.h"
#include "ray.h"


//TODO: Move to primitives header file
struct Sphere {
	vec3 center;
	float radius;
};

vec3 color(const ray& r) {
	vec3 unit_direction = unit_vector(r.direction());
	//Scale to 0.0 - 1.0
	float t = 0.5 * (unit_direction.y() + 1.0);

	//Linearly blend blue and white
	return (1.0 - t) * vec3(1.0, 1.0, 1.0) + t * vec3(0.5, 0.7, 1.0);
}

bool hit_sphere(const Sphere sphere, const ray& r) {
	
}

int main() {
	std::ofstream file;
	const int ROWS = 100;
	const int COLUMNS = 200;
	vec3 origin(0.0, 0.0, 0.0);
	vec3 vertical(0.0, 2.0, 0.0);
	vec3 horizontal(4.0, 0.0, 0.0);
	vec3 ll_corner(-2.0, -1.0, -1.0);

	file.open("color_test.ppm");
	file << "P3\n" << COLUMNS << " " << ROWS << "\n255\n";
	for (int row = ROWS - 1; row >=0; row--) {
		for (int col = 0; col < COLUMNS; col++) {
			vec3 v { float(col) / float(COLUMNS), float(row) / float(ROWS), 0.2 };
			int ir = int(v.x() * 255.99);
			int ig = int(v.y() * 255.99);
			int ib = int(v.z() * 255.99);
			file << ir << " " << ig << " " << ib << "\n";
		}
	}
	file.close();
	
	file.open("ray_test.ppm");
	file << "P3\n" << COLUMNS << " " << ROWS << "\n255\n";
	for (int row = ROWS - 1; row >= 0; row--) {
		for (int col = 0; col < COLUMNS; col++) {
			ray r (origin, ll_corner + float(col) / float(COLUMNS) * horizontal + float(row) / float(ROWS) * vertical);
			vec3 v = color(r);
			int ir = int(v.x() * 255.99);
			int ig = int(v.y() * 255.99);
			int ib = int(v.z() * 255.99);
			file << ir << " " << ig << " " << ib << "\n";
		}
	}
	file.close();

	file.open("sphere_test.ppm");
	for (int row = ROWS - 1; row >= 0; row--) {
		for (int col = 0; col < COLUMNS; col++) {
			ray r (origin, ll_corner + float(col) / float(COLUMNS) * horizontal + float(row) / float(ROWS) * vertical);
			vec3 v = color(r);
			int ir = int(v.x() * 255.99);
			int ig = int(v.y() * 255.99);
			int ib = int(v.z() * 255.99);
			file << ir << " " << ig << " " << ib << "\n";
		}
	}
}
