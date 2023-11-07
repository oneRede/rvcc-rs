void assert(int expected, int actual, char *code);

int main()
{
   assert(1, ({ int x[3]={1,2,3}; x[0]; }));
}