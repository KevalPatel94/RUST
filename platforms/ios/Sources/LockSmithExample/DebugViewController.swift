import UIKit
import SwiftUI

final class DebugViewController: UITableViewController {
    private enum DebugItem: String, CaseIterable {
        case locksmith
        case userListRxSwift
        case userListSwiftUI
        
        var title: String {
            switch self {
            case .locksmith:
                return "LockSmith"
            case .userListRxSwift:
                return "User List (RxSwift)"
            case .userListSwiftUI:
                return "User List (SwiftUI)"
            }
        }
    }
    
    override func viewDidLoad() {
        super.viewDidLoad()
        title = "Debug Menu"
        tableView.register(UITableViewCell.self, forCellReuseIdentifier: "Cell")
    }
    
    override func tableView(_ tableView: UITableView, numberOfRowsInSection section: Int) -> Int {
        DebugItem.allCases.count
    }
    
    override func tableView(_ tableView: UITableView, cellForRowAt indexPath: IndexPath) -> UITableViewCell {
        let cell = tableView.dequeueReusableCell(withIdentifier: "Cell", for: indexPath)
        let item = DebugItem.allCases[indexPath.row]
        cell.textLabel?.text = item.title
        cell.accessoryType = .disclosureIndicator
        return cell
    }
    
    override func tableView(_ tableView: UITableView, didSelectRowAt indexPath: IndexPath) {
        tableView.deselectRow(at: indexPath, animated: true)
        let item = DebugItem.allCases[indexPath.row]
        
        switch item {
        case .locksmith:
            let passwordViewController = ViewController()
            navigationController?.pushViewController(passwordViewController, animated: true)
        case .userListRxSwift:
            let userListViewController = UserListViewControllerRx()
            navigationController?.pushViewController(userListViewController, animated: true)
        case .userListSwiftUI:
            let swiftUIView = UserListViewSwiftUI()
            let hostingController = UIHostingController(rootView: swiftUIView)
            navigationController?.pushViewController(hostingController, animated: true)
        }
    }
}






