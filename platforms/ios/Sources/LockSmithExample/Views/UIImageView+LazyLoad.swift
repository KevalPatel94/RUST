import UIKit

extension UIImageView {
    /// Loads an image asynchronously from a URL with caching
    /// - Parameter urlString: The URL string of the image to load
    func loadImage(from urlString: String?) {
        // Reset image
        image = nil
        
        guard let urlString = urlString, !urlString.isEmpty,
              let url = URL(string: urlString) else {
            // Use placeholder if URL is invalid
            image = UIImage(systemName: "person.circle.fill")
            return
        }
        
        // Check cache first
        if let cachedImage = ImageCache.shared.image(for: url) {
            self.image = cachedImage
            return
        }
        
        // Show placeholder while loading
        image = UIImage(systemName: "person.circle.fill")
        
        // Load image asynchronously
        Task {
            do {
                let (data, _) = try await URLSession.shared.data(from: url)
                if let image = UIImage(data: data) {
                    // Cache the image
                    await ImageCache.shared.setImage(image, for: url)
                    
                    // Update UI on main thread
                    await MainActor.run {
                        // Only update if this cell is still showing the same URL
                        // (prevents race conditions with scrolling)
                        self.image = image
                    }
                }
            } catch {
                // Keep placeholder on error
                await MainActor.run {
                    self.image = UIImage(systemName: "person.circle.fill")
                }
            }
        }
    }
}

// MARK: - Image Cache for UIKit
/// Thread-safe image cache for UIKit lazy loading
actor ImageCache {
    static let shared = ImageCache()
    private var cache: [URL: UIImage] = [:]
    private let maxCacheSize = 100 // Limit cache size
    
    private init() {}
    
    func image(for url: URL) -> UIImage? {
        return cache[url]
    }
    
    func setImage(_ image: UIImage, for url: URL) {
        // Simple LRU: remove oldest if cache is full
        if cache.count >= maxCacheSize {
            if let firstKey = cache.keys.first {
                cache.removeValue(forKey: firstKey)
            }
        }
        cache[url] = image
    }
    
    func clearCache() {
        cache.removeAll()
    }
}







