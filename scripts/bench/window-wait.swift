// Launch an app through Launch Services and report when its first window
// is on screen (docs/PERFORMANCE.md). The same yardstick for every
// editor, Text³ included: `open -n -a <app> <args>` (a new instance, as
// a double-click would start it), then the window server is polled every
// millisecond for a normal-layer window larger than 100×100 owned by a
// new process inside the app bundle (Electron apps draw in a helper).
// No in-app probe. Reads only window owners and bounds, which need no
// screen-recording permission.
//
// Usage: window-wait <timeout-s> <App.app> [open arguments...]
// Prints "pid=<pid> window_ms=<ms>" or "pid=0 window_ms=timeout". The app
// keeps running; the caller ends it.
import CoreGraphics
import Darwin
import Foundation

let args = CommandLine.arguments
guard args.count >= 3, let timeout = Double(args[1]) else {
    FileHandle.standardError.write("usage: window-wait <timeout-s> <App.app> [open args...]\n".data(using: .utf8)!)
    exit(2)
}
let bundle = URL(fileURLWithPath: args[2]).standardizedFileURL.resolvingSymlinksInPath().path + "/"

func path(of pid: pid_t) -> String? {
    var buf = [CChar](repeating: 0, count: 4 * Int(MAXPATHLEN))
    let n = proc_pidpath(pid, &buf, UInt32(buf.count))
    return n > 0 ? String(cString: buf) : nil
}

func appPids() -> Set<pid_t> {
    let n = proc_listallpids(nil, 0)
    var pids = [pid_t](repeating: 0, count: Int(n) * 2)
    let got = proc_listallpids(&pids, Int32(pids.count * MemoryLayout<pid_t>.size))
    return Set(pids.prefix(Int(got)).filter { $0 > 0 && (path(of: $0)?.hasPrefix(bundle) ?? false) })
}

let before = appPids()
let start = DispatchTime.now()
let open = Process()
open.executableURL = URL(fileURLWithPath: "/usr/bin/open")
open.arguments = ["-n", "-a", args[2]] + Array(args.dropFirst(3))
do { try open.run() } catch {
    FileHandle.standardError.write("cannot run open: \(error)\n".data(using: .utf8)!)
    exit(1)
}
var known: [pid_t: Bool] = [:]  // pid → inside the bundle and new
while true {
    let elapsed = Double(DispatchTime.now().uptimeNanoseconds - start.uptimeNanoseconds) / 1e6
    if elapsed > timeout * 1000 {
        print("pid=0 window_ms=timeout")
        exit(0)
    }
    let list = CGWindowListCopyWindowInfo([.optionOnScreenOnly, .excludeDesktopElements], kCGNullWindowID) as? [[String: Any]] ?? []
    for w in list {
        guard let owner = w[kCGWindowOwnerPID as String] as? Int32,
              (w[kCGWindowLayer as String] as? Int) == 0,
              let b = w[kCGWindowBounds as String] as? [String: Double],
              (b["Width"] ?? 0) > 100, (b["Height"] ?? 0) > 100
        else { continue }
        let ours = known[owner] ?? {
            let v = !before.contains(owner) && (path(of: owner)?.hasPrefix(bundle) ?? false)
            known[owner] = v
            return v
        }()
        if ours {
            print(String(format: "pid=%d window_ms=%.1f", owner, elapsed))
            exit(0)
        }
    }
    usleep(1000)
}
