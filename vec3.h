#include <math.h>
#include <stdlib.h>
#include <iostream>

class vec3 {
	public:
		vec3() {}
		vec3(float x, float y, float z) { d[0] = x; d[1] = y; d[2] = z; }

		inline float x() const { return d[0]; }
		inline float y() const { return d[1]; }
		inline float z() const { return d[2]; }
		inline float r() const { return d[0]; }
		inline float g() const { return d[1]; }
		inline float b() const { return d[2]; }

		inline vec3 operator-() const { return vec3 { -d[0], -d[1], -d[2] };}
		inline float operator[](int i) { return d[i]; }

		inline vec3& operator+=(const vec3 &v2);
		inline vec3& operator-=(const vec3 &v2);
		inline vec3& operator*=(const vec3 &v2);
		inline vec3& operator/=(const vec3 &v2);
		inline vec3& operator*=(const float t);
		inline vec3& operator/=(const float t);

		inline float squared_length() const {
			return d[0] * d[0] + d[1] * d[1] + d[2] * d[2];
		}
		inline float length() const {
			return sqrt(squared_length());
		}
		inline void make_unit_vector();	
		float d[3];
};

inline void vec3::make_unit_vector() { 
	float m = 1.0 / squared_length();
	d[0] *= m; d[1] *= m; d[2] *= m;
}
inline std::istream& operator>>(std::istream &is, vec3 &t) {
	is >> t.d[0] >> t.d[1] >> t.d[2];
	return is;
}
inline std::ostream& operator>>(std::ostream &os, vec3 &t) {
	os << t.d[0] << " " <<  t.d[1] << " " << t.d[2];
	return os;
}
inline vec3 operator+(const vec3 &v1, const vec3 &v2){
	return vec3 { v1.d[0] + v2.d[0], v1.d[1] + v2.d[1], v1.d[2] + v2.d[2] };
}
inline vec3 operator-(const vec3 &v1, const vec3 &v2){
	return vec3 { v1.d[0] - v2.d[0], v1.d[1] - v2.d[1], v1.d[2] - v2.d[2] };
}
inline vec3 operator*(const vec3 &v1, const vec3 &v2){
	return vec3 { v1.d[0] * v2.d[0], v1.d[1] * v2.d[1], v1.d[2] * v2.d[2] };
}
inline vec3 operator/(const vec3 &v1, const vec3 &v2){
	return vec3 { v1.d[0] / v2.d[0], v1.d[1] / v2.d[1], v1.d[2] / v2.d[2] };
}
inline vec3 operator*(float t, const vec3 &v2){
	return vec3 { t * v2.d[0], t * v2.d[1], t * v2.d[2] };
}
inline vec3 operator/(const vec3 &v2, float t){
	return vec3 { v2.d[0] / t, v2.d[1] / t, v2.d[2] / t };
}
inline vec3 operator*(const vec3 &v2, float t){
	return vec3 { t * v2.d[0], t * v2.d[1], t * v2.d[2] };
}
inline float dot(const vec3 &v1, const vec3 &v2) {
	return v1.d[0] * v2.d[0] + v1.d[1] * v2.d[1] + v1.d[2] + v2.d[2];
}
inline vec3 cross(const vec3 &v1, const vec3 &v2) {
	return vec3 { v1.d[1] * v2.d[2] - v1.d[2] * v2.d[1], 
			-v1.d[0] * v2.d[2] + v1.d[2] * v2.d[0], 
			v1.d[0] * v2.d[1] - v1.d[1] * v2.d[0] };
}
inline vec3& vec3::operator+=(const vec3 &v2){
	d[0] += v2.d[0];
	d[1] += v2.d[1];
	d[2] += v2.d[2];
	return *this;
}
inline vec3& vec3::operator-=(const vec3 &v2){
	d[0] -= v2.d[0];
	d[1] -= v2.d[1];
	d[2] -= v2.d[2];
	return *this;
}
inline vec3& vec3::operator*=(const vec3 &v2){
	d[0] *= v2.d[0];
	d[1] *= v2.d[1];
	d[2] *= v2.d[2];
	return *this;
}
inline vec3& vec3::operator/=(const vec3 &v2){
	d[0] /= v2.d[0];
	d[1] /= v2.d[1];
	d[2] /= v2.d[2];
	return *this;
}
inline vec3& vec3::operator*=(const float t){
	d[0] *= t;
	d[1] *= t;
	d[2] *= t;
	return *this;
}
inline vec3& vec3::operator/=(const float t){
	d[0] /= t;
	d[1] /= t;
	d[2] /= t;
	return *this;
}


