int main()
{
   assert(1, ({ typedef int t; t x=1; x; }));
}