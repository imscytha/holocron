#include <stdio.h>

// NOTE: defines function pointer as a type
typedef void (*TestCallbackFn)();

void hello()
{
    printf("Hello!\n");
}

void bye()
{
    printf("Bye!\n");
}

void acceptsCallback(TestCallbackFn callback)
{
    printf("About to run callback:\n");
    if (callback != NULL) {
        callback();
    }
}

int main()
{
    printf("Hello, World\n");

    acceptsCallback(hello);
    acceptsCallback(bye);
    acceptsCallback(NULL);
    return 0;
}
