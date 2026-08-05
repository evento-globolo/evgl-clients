// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "EvglClient",
    platforms: [.macOS(.v12), .iOS(.v15)],
    products: [.library(name: "EvglClient", targets: ["EvglClient"])],
    targets: [
        .target(name: "EvglClient"),
        .testTarget(name: "EvglClientTests", dependencies: ["EvglClient"])
    ]
)
