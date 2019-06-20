#include <iostream>
#include <fstream>
#include <limits>

#include "primitives.h"
#include "camera.h"
#include "utility.h"
#include "material.h"


vec3 color(const ray& r, Collideable *world, int depth) {
	HitRecord rec;
	if (world->hit(r, 0.001, std::numeric_limits<float>::max(), rec)) {
		ray scattered;
		vec3 attenuation;
		if (depth < 50 && rec.mat_ptr->scatter(r, rec, attenuation, scattered)) {
			return attenuation * color (scattered, world, depth + 1);
		}
		return vec3(0, 0, 0);
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
	srand(time(NULL));
	Collideable* list[4];
	list[0] = new Sphere(vec3(0, 0, -1), 0.5, new Lambertian(vec3(0.8, 0.3, 0.3)));
	list[1] = new Sphere(vec3(0, -100.5, -1), 100, new Lambertian(vec3(0.8, 0.8, 0.0)));
	list[2] = new Sphere(vec3(-1, 0, -1), 0.5, new Dielectric(1.5));
	list[3] = new Sphere(vec3(1, 0, -1), 0.5, new Metal(vec3(0.8, 0.8, 0.8), 0.3));
	Collideable* world = new CollideableList(list, 4);
	Camera camera;
	
	file.open("ray_test.ppm");
	std::cout << "Writing to file..." << std::endl;
	file << "P3\n" << COLUMNS << " " << ROWS << "\n255\n";
	std::cout << "Generating colors..." << std::endl;
	for (int row = ROWS - 1; row >= 0; row--) {
		for (int col = 0; col < COLUMNS; col++) {
			vec3 averaged_color (0, 0, 0);
			for (int s = 0; s < SAMPLES; s++) {
				float u = float(col + get_random_float()) / float(COLUMNS);
				float v = float(row + get_random_float()) / float(ROWS);
				ray r = camera.get_ray(u, v);
				averaged_color += color(r, world, 0);
			}
			averaged_color /= float(SAMPLES);
			averaged_color = vec3( sqrt(averaged_color.x()), sqrt(averaged_color.y()) ,sqrt(averaged_color.z()));
			int ir = int(averaged_color.x() * 255.99);
			int ig = int(averaged_color.y() * 255.99);
			int ib = int(averaged_color.z() * 255.99);
			file << ir << " " << ig << " " << ib << "\n";
		}
	}
	std::cout << "Closing file..." << std::endl;
	file.flush();
	file.close();
}
