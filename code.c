typedef int MyInt;

int main()
{
   assert(3, ({ MyInt x=3; x; }));
}