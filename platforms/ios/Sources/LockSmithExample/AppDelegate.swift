import UIKit

@main
class AppDelegate: UIResponder, UIApplicationDelegate {
    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        let window = UIWindow(frame: UIScreen.main.bounds)
        let debugViewController = DebugViewController()
        let navigationController = UINavigationController(rootViewController: debugViewController)
        window.rootViewController = navigationController
        window.makeKeyAndVisible()
        self.window = window
        return true
    }
}

