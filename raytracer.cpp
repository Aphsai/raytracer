#include <iostream>
#include <fstream>
#include <limits>

#include "primitives.h"

vec3 color(ray &r, Entity* shape) {}

int main() {
	std::ofstream file;
	const int HEIGHT = 720;
	const int WIDTH = 1280;
	const int SAMPLES = 100;
	srand(time(NULL));

	file.open("ray_test.ppm");
	std::cout << "Writing to file..." << std::endl;
	file << "P3\n" << WIDTH << " " << HEIGHT << "\n255\n";
	std::cout << "Generating pixels..." << std::endl;
    Entity* sphere = new Sphere( vec3(0, 0, 0), 0.5 );
	for (int row = HEIGHT - 1; row >= 0; row--) {
		for (int col = 0; col < WIDTH; col++) {
			vec3 averaged_color (0, 0, 0);
			for (int s = 0; s < SAMPLES; s++) {
                ray r;
				averaged_color += color(r, sphere);
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
