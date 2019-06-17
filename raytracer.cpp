#include <iostream>
#include <fstream>
#include "vec3.h"

int main() {
	std::ofstream file;
	file.open("test.ppm");
	const int ROWS = 100;
	const int COLUMNS = 200;
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
}
