#include "test.h"

int main() {
  // [97] 支持局部变量初始化器\n
  ASSERT(1, ({ int x[3]={1,2,3}; x[0]; }));

  printf("OK\n");
  return 0;
}
