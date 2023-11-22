void assert(int expected, int actual, char *code);

int main()
{
   assert(0x01020304, ({ union { struct { char a,b,c,d; } e; int f; } x={{4,3,2,1}}; x.f; }));
}