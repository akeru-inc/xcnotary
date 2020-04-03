import Cocoa

final class AppDelegate: NSObject, NSApplicationDelegate {
    @IBOutlet weak var window: NSWindow!

    func applicationDidFinishLaunching(_ aNotification: Notification) {
        let mainMenu = NSMenu()

        let appMenu = NSMenu()
        let appMenuItem = mainMenu.addItem(withTitle: "", action: nil, keyEquivalent: "")
        appMenuItem.submenu = appMenu
        appMenu.addItem(withTitle: "Quit", action: #selector(NSApplication.terminate(_:)), keyEquivalent: "q")
        
        NSApplication.shared.mainMenu = mainMenu
    }

    func applicationWillTerminate(_ aNotification: Notification) {
        // Insert code here to tear down your application
    }
}

let app: NSApplication = NSApplication.shared
let appDelegate = AppDelegate()
app.delegate = appDelegate

_ = NSApplicationMain(CommandLine.argc, CommandLine.unsafeArgv)


