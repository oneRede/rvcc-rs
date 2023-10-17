void assert(int expected, int actual, char *code);

int main()
{
   assert(1, ({ int i=1; i<<=0; i; }));
}