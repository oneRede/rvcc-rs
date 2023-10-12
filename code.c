void assert(int expected, int actual, char *code);

int main()
{
   assert(1, ({ int i=0; goto i; g: i++; h: i++; i: i++; i; }));
}