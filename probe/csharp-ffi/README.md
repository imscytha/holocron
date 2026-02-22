# C# FFI

I found myself wondering the capabilities of C# for calling native libs. This
probe serves as a playground for that.

Digging through the [docs][native-interop], I found that [P/Invoke][p-invoke]
is the engine that powers most interop scenarios. What really stood out was the
developer experience. With little to no knowledge of FFIs and native function
calls, it was relatively easy to do this in C#.

Through source generation, you can write the C function signature you want to
bind against in C# (using C# primitives) and decorate it with
[`LibraryImport`][library-import] attribute. The source generator then
automatically handles the library loading and data marshalling.

Example:

For the following C function

```c
void InitWindow(int width, int height, const char *title);
```

We just adapt to the respective C# primitives and decorate the method as fore
mentioned:

```cs
[LibraryImport(RAYLIB_PATH, StringMarshalling = StringMarshalling.Utf8)]
internal static partial void InitWindow(int width, int height, string title);
```

> [!NOTE]
> Since `[LibraryImport]` uses a source generator to create the marshalling
> logic at compile-time rather than runtime, we must explicitly specify to the
> compiler how the string argument should be marshalled for the generated
> `p/invoke` via the `StringMarshalling` property.

## Tools used

- [**.NET 10.0 SDK**][dotnet-sdk]
- [**raylib**][raylib]

## Build & Run

```bash
dotnet build
dotnet run
```

A small window should appear with "This is called from C#!" in the title bar.

[native-interop]: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/
[p-invoke]: https://learn.microsoft.com/en-us/dotnet/standard/native-interop/pinvoke
[library-import]: https://learn.microsoft.com/en-us/dotnet/api/system.runtime.interopservices.libraryimportattribute?view=net-10.0
[dotnet-sdk]: https://dotnet.microsoft.com/en-us/download/dotnet/10.0
[raylib]: ./raylib-5.5_macos/
