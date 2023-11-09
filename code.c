void assert(int expected, int actual, char *code);

int main()
{
   assert(4, ({ int x[]={1,2,3,4}; x[3]; }));
}