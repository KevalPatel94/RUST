import SwiftUI
import LockSmith

/// SwiftUI implementation of User List View
struct UserListViewSwiftUI: View {
    @StateObject private var viewModel = UserListViewModelSwiftUI()
    
    var body: some View {
        NavigationView {
            ZStack {
                if viewModel.isLoading && viewModel.users.isEmpty {
                    ProgressView()
                        .scaleEffect(1.5)
                } else if viewModel.showError {
                    errorView
                } else if viewModel.users.isEmpty {
                    emptyView
                } else {
                    userListView
                }
            }
            .navigationTitle("Users (SwiftUI)")
            .refreshable {
                viewModel.refresh()
            }
            .onAppear {
                if viewModel.users.isEmpty {
                    viewModel.loadUsers()
                }
            }
        }
    }
    
    // MARK: - Subviews
    private var userListView: some View {
        List(viewModel.users) { user in
            UserRowView(user: user)
        }
    }
    
    private var emptyView: some View {
        VStack(spacing: 16) {
            Image(systemName: "person.3.fill")
                .font(.system(size: 50))
                .foregroundColor(.secondary)
            Text("No users found")
                .font(.title3)
                .foregroundColor(.secondary)
        }
    }
    
    private var errorView: some View {
        VStack(spacing: 16) {
            Image(systemName: "exclamationmark.triangle.fill")
                .font(.system(size: 50))
                .foregroundColor(.red)
            
            Text(viewModel.errorMessage ?? "Unknown error")
                .font(.body)
                .foregroundColor(.secondary)
                .multilineTextAlignment(.center)
                .padding(.horizontal)
            
            Button("Retry") {
                viewModel.loadUsers()
            }
            .buttonStyle(.borderedProminent)
        }
        .padding()
    }
}

// MARK: - User Row View
private struct UserRowView: View {
    let user: UserPresentationModel
    
    var body: some View {
        VStack(alignment: .leading, spacing: 8) {
            Text(user.displayName)
                .font(.headline)
            
            Text(user.email)
                .font(.subheadline)
                .foregroundColor(.secondary)
            
            Text(user.ageDisplay)
                .font(.caption)
                .foregroundColor(.white)
        }
        .padding(.vertical, 4)
    }
}

// MARK: - Preview
struct UserListViewSwiftUI_Previews: PreviewProvider {
    static var previews: some View {
        UserListViewSwiftUI()
    }
}


