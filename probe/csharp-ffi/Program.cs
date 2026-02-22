namespace csharp_ffi;

internal class Program
{
    private static void Main(string[] args)
    {
        Raylib.SetTraceLogLevel(TraceLogLevel.WARNING);
        Raylib.InitWindow(200, 300, "This is called from C#!");
#if DEBUG
        // NOTE: test flags
        ConfigFlags[] windowFlags = [ ConfigFlags.WINDOW_RESIZABLE, ConfigFlags.WINDOW_MINIMIZED ];
        Raylib.SetWindowState(windowFlags);
        Raylib.SetTraceLogLevel(TraceLogLevel.TRACE);
#endif

        while (!Raylib.WindowShouldClose())
        {
            Raylib.BeginDrawing();
            Raylib.TraceLog(TraceLogLevel.WARNING, "Hello from C#!");
            Raylib.EndDrawing();
        }
    }
}
