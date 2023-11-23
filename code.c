void assert(int expected, int actual, char *code);

struct {int a[2];} g12[2] = {{{1, 2}}};
int main()
{
   assert(0, g12[1].a[0]);
}