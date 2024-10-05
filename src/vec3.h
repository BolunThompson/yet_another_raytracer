#pragma once

#include <cmath>
#include <iostream>

class vec3 {
public:
  float e[3];

  constexpr vec3() : e{0, 0, 0} {}
  constexpr vec3(float e0, float e1, float e2) : e{e0, e1, e2} {}

  constexpr float x() const { return e[0]; }
  constexpr float y() const { return e[1]; }
  constexpr float z() const { return e[2]; }

  constexpr vec3 operator-() const { return vec3(-e[0], -e[1], -e[2]); }
  constexpr float operator[](int i) const { return e[i]; }
  constexpr float &operator[](int i) { return e[i]; }

  constexpr vec3 &operator+=(const vec3 &v) {
    e[0] += v.e[0];
    e[1] += v.e[1];
    e[2] += v.e[2];
    return *this;
  }

  constexpr vec3 &operator*=(float t) {
    e[0] *= t;
    e[1] *= t;
    e[2] *= t;
    return *this;
  }

  constexpr vec3 &operator/=(float t) { return *this *= 1 / t; }

  constexpr float length() const { return std::sqrt(length_squared()); }

  constexpr float length_squared() const {
    return e[0] * e[0] + e[1] * e[1] + e[2] * e[2];
  }
};

// point3 is just an alias for vec3, but useful for geometric clarity in the
// code.
using point3 = vec3;

// Vector Utility Functions

inline std::ostream &operator<<(std::ostream &out, const vec3 &v) {
  return out << v.e[0] << ' ' << v.e[1] << ' ' << v.e[2];
}

inline constexpr vec3 operator+(const vec3 &u, const vec3 &v) {
  return vec3(u.e[0] + v.e[0], u.e[1] + v.e[1], u.e[2] + v.e[2]);
}

inline constexpr vec3 operator-(const vec3 &u, const vec3 &v) {
  return vec3(u.e[0] - v.e[0], u.e[1] - v.e[1], u.e[2] - v.e[2]);
}

inline constexpr vec3 operator*(const vec3 &u, const vec3 &v) {
  return vec3(u.e[0] * v.e[0], u.e[1] * v.e[1], u.e[2] * v.e[2]);
}

inline constexpr vec3 operator*(float t, const vec3 &v) {
  return vec3(t * v.e[0], t * v.e[1], t * v.e[2]);
}

inline constexpr vec3 operator*(const vec3 &v, float t) { return t * v; }

inline constexpr vec3 operator/(const vec3 &v, float t) { return (1 / t) * v; }

inline constexpr float dot(const vec3 &u, const vec3 &v) {
  return u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2];
}

inline constexpr vec3 cross(const vec3 &u, const vec3 &v) {
  return vec3(u.e[1] * v.e[2] - u.e[2] * v.e[1],
              u.e[2] * v.e[0] - u.e[0] * v.e[2],
              u.e[0] * v.e[1] - u.e[1] * v.e[0]);
}

inline constexpr vec3 unit_vector(const vec3 &v) { return v / v.length(); }