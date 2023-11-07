void assert(int expected, int actual, char *code);

int main()
{
   assert(0, ({ int x[2][3]={{1,2}}; x[1][2]; }));
}