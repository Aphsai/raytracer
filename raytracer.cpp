#include <iostream>
#include <fstream>

int main() {
	std::ofstream file;
	file.open("test.ppm");
	const int ROWS = 100;
	const int COLUMNS = 200;
	file << "P3\n" << COLUMNS << " " << ROWS << "\n255\n";
	for (int row = ROWS - 1; row >=0; row--) {
		for (int col = 0; col < COLUMNS; col++) {
			float r = float(col) / float(COLUMNS);
			float g = float(row) / float(ROWS);
			float b = 0.2;

			int ir = int(255.99 * r);
			int ig = int(255.99 * g);
			int ib = int(255.99 * b);

			file << ir << " " << ig << " " << ib << "\n";
		}
	}
	file.close();
}
