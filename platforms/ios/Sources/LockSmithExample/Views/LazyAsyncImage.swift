import SwiftUI

/// A lazy-loading image component that loads images asynchronously
/// with placeholder, error handling, and caching support
struct LazyAsyncImage<Content: View, Placeholder: View>: View {
    let url: String?
    let content: (Image) -> Content
    let placeholder: () -> Placeholder
    
    @State private var loadedImage: UIImage?
    @State private var isLoading = true
    @State private var hasError = false
    
    init(
        url: String?,
        @ViewBuilder content: @escaping (Image) -> Content,
        @ViewBuilder placeholder: @escaping () -> Placeholder
    ) {
        self.url = url
        self.content = content
        self.placeholder = placeholder
    }
    
    var body: some View {
        Group {
            if let image = loadedImage {
                content(Image(uiImage: image))
            } else if isLoading {
                placeholder()
            } else if hasError {
                // Fallback to placeholder on error
                placeholder()
            } else {
                placeholder()
            }
        }
        .task {
            await loadImage()
        }
    }
    
    @MainActor
    private func loadImage() async {
        guard let urlString = url, !urlString.isEmpty,
              let imageURL = URL(string: urlString) else {
            isLoading = false
            hasError = true
            return
        }
        
        // Check cache first
        if let cachedImage = await SwiftUIImageCache.shared.image(for: imageURL) {
            loadedImage = cachedImage
            isLoading = false
            return
        }
        
        isLoading = true
        hasError = false
        
        do {
            let (data, _) = try await URLSession.shared.data(from: imageURL)
            if let image = UIImage(data: data) {
                // Cache the image
                await SwiftUIImageCache.shared.setImage(image, for: imageURL)
                loadedImage = image
                hasError = false
            } else {
                hasError = true
            }
        } catch {
            hasError = true
        }
        
        isLoading = false
    }
}

// MARK: - Image Cache
/// Simple in-memory image cache for lazy-loaded images
actor SwiftUIImageCache {
    static let shared = SwiftUIImageCache()
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

// MARK: - Convenience Extensions
extension LazyAsyncImage where Content == Image, Placeholder == ProgressView<EmptyView, EmptyView> {
    /// Convenience initializer with default placeholder
    init(url: String?) {
        self.init(
            url: url,
            content: { $0 },
            placeholder: { ProgressView() }
        )
    }
}

extension LazyAsyncImage where Content == Image {
    /// Convenience initializer with custom placeholder
    init(url: String?, @ViewBuilder placeholder: @escaping () -> Placeholder) {
        self.init(
            url: url,
            content: { $0 },
            placeholder: placeholder
        )
    }
}







