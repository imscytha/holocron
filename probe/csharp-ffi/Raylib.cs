using System.Runtime.InteropServices;

namespace csharp_ffi;

/// <summary>
/// Static wrapper for the raylib C library.
/// </summary>
public static partial class Raylib
{
    private const string RAYLIB_PATH = "./raylib-5.5_macos/lib/libraylib.dylib";

    /// <summary>
    /// Initializes the window and OpenGL context.
    /// </summary>
    /// <param name="width">Width of the window in pixels.</param>
    /// <param name="height">Height of the window in pixels.</param>
    /// <param name="title">Title displayed at the top of the window.</param>
    [LibraryImport(RAYLIB_PATH, StringMarshalling = StringMarshalling.Utf8)]
    internal static partial void InitWindow(int width, int height, string title);

    /// <summary>
    /// Checks if the 'Close' button or ESC key has been pressed.
    /// </summary>
    /// <returns>True if the window should close; otherwise, false.</returns>
    [LibraryImport(RAYLIB_PATH)]
    [return: MarshalAs(UnmanagedType.I1)]
    internal static partial bool WindowShouldClose();

    /// <summary>
    /// Sets up the canvas (framebuffer) to start drawing.
    /// </summary>
    [LibraryImport(RAYLIB_PATH)]
    internal static partial void BeginDrawing();

    /// <summary>
    /// Ends canvas drawing and swaps the back buffer to the screen (Double buffering).
    /// </summary>
    [LibraryImport(RAYLIB_PATH)]
    internal static partial void EndDrawing();

    /// <summary>
    /// Directly sets a specific combination of configuration flags for the window.
    /// </summary>
    /// <param name="flags">The bitwise ORed configuration flags.</param>
    [LibraryImport(RAYLIB_PATH)]
    private static partial void SetWindowState(ConfigFlags flags);
    /// <summary>
    /// Sets one or more configuration flags for the window using a params array.
    /// </summary>
    /// <param name="flags">A list of <see cref="ConfigFlags"/> to apply.</param>
    internal static void SetWindowState(params ConfigFlags[] flags)
        => SetWindowState(flags.Aggregate(ConfigFlags.NOOP, (acc, flag) => acc | flag));

    /// <summary>
    /// Sets the minimum severity level for log messages to be displayed.
    /// </summary>
    /// <param name="level">The <see cref="TraceLogLevel"/> threshold.</param>
    [LibraryImport(RAYLIB_PATH)]
    internal static partial void SetTraceLogLevel(TraceLogLevel level);

    /// <summary>
    /// Print a custom trace log message (INFO, WARNING, ERROR, etc.).
    /// </summary>
    /// <param name="level">The severity level of the log.</param>
    /// <param name="message">The message to log.</param>
    [LibraryImport(RAYLIB_PATH, StringMarshalling = StringMarshalling.Utf8)]
    internal static partial void TraceLog(TraceLogLevel level, string message);
}

/// <summary>
/// Configuration flags used to setup System/Window state.
/// </summary>
public enum ConfigFlags : int
{
    /// <summary>No flags set.</summary>
    NOOP             = 0x00000000,
    /// <summary>Set to allow resizable window.</summary>
    WINDOW_RESIZABLE = 0x00000004,
    /// <summary>Set to minimize window (iconify) on startup.</summary>
    WINDOW_MINIMIZED = 0x00000200
}

/// <summary>
/// Logging levels for the TraceLog function.
/// </summary>
public enum TraceLogLevel : int
{
/// <summary>Display all logs.</summary>
    ALL = 0,
    /// <summary>Trace logging, intended for internal use only.</summary>
    TRACE,
    /// <summary>Debug logging, used for internal debugging; should be disabled in release builds.</summary>
    DEBUG,
    /// <summary>Information logging, used for program execution info.</summary>
    INFO,
    /// <summary>Warning logging, used on recoverable failures.</summary>
    WARNING,
    /// <summary>Error logging, used on unrecoverable failures.</summary>
    ERROR,
    /// <summary>Fatal logging, used to abort program.</summary>
    FATAL,
    /// <summary>Disable all logging.</summary>
    NONE
}
