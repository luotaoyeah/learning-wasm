// compiling-c-or-c++-as-a-side-module-with-emscripten

/**
 * $ emcc side-module.cc -s SIDE_MODULE=2 -O1 -s "EXPORTED_FUNCTIONS=['_Increment']" -o side-module.wasm
 *
 * `-o side-module.wasm`                    表示只生成 wasm 文件
 * `-O1`                                    表示使用一级优化, 移除一些不必要的代码
 * `-s SIDE_MODULE=2`                       表示生成一个 side-module, 而不是一个 main-module
 * `-s "EXPORTED_FUNCTIONS=['_Increment']"` 表示指定要输出的函数
 */
int Increment(int i) {
  return i + 1;
}
