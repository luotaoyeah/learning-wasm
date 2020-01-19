#include <emscripten.h>

#include <cstdio>

int IsPrime(int value) {
  if (value == 2) {
    return 1;
  }
  if (value <= 1 || value % 2 == 0) {
    return 0;
  }
  for (int i = 3; (i * i) <= value; i += 2) {
    if (value % i == 0) {
      return 0;
    }
  }
  return 1;
}

/**
 * $ emcc calculate-primes.cc -o calculate-primes.js
 *
 * `-o <target>` 指定输出的文件名, 文件名的后缀决定了输出文件的类型,
 * '.js' 表示生成 wasm + js 两个文件
 */
int main() {
  int start = 3;
  int end = 100000;

  printf("prime numbers between %d and %d:\n", start, end);

  for (int i = start; i <= end; i += 2) {
    if (IsPrime(i)) {
      printf("%d ", i);
    }
  }
  printf("\n");

  return 0;
}