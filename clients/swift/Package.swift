// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "EvglClient",
    platforms: [.iOS(.v15), .macOS(.v12)],
    products: [.library(name: "EvglClient", targets: ["EvglClient"])],
    targets: [.target(name: "EvglClient")]
)
